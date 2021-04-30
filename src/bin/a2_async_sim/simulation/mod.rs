use agents::app::UpdateTimer;
use anyhow::Result;
use std::{
    sync::mpsc::{self, Sender, TryRecvError},
    thread,
    time::Duration,
};
use triple_buffer::{Input, Output, TripleBuffer};

/// A simulation is a type which owns a persistent state which it updates
/// asynchronously.
///
/// Updates are made available to the main thread via a triple buffer which
/// allows the simulation and main thread to update at different rates without
/// blocking.
pub trait Simulation<State: Send> {
    /// Initialize the simulation. Called once right before starting the main
    /// loop.
    fn setup(&mut self, sync: &mut Input<State>);

    /// Advance the simulation by one time step (fixed steps).
    fn tick(&mut self, sync: &mut Input<State>, duration: Duration);
}

/// A Simulation worker owns a thread which runs a simulation asynchronously.
pub struct Worker<State: Send> {
    join_handle: Option<thread::JoinHandle<()>>,
    stop_sender: Sender<()>,
    output_buffer: Output<State>,
}

impl<T: 'static + Send + Clone + Default> Worker<T> {
    /// Create a new simulation worker instance.
    ///
    /// # Note
    ///
    /// The state must be default-constructable to initialize the triple buffer.
    pub fn new<S: 'static + Simulation<T>>(
        throttle: Duration,
        create: fn() -> S,
    ) -> Result<Self> {
        // a channel is used to signal when the worker should terminate
        let (stop_sender, stop_receiver) = mpsc::channel();

        // a triple-buffer with initial data set to the default
        let (mut input_buffer, output_buffer) =
            TripleBuffer::new(T::default()).split();

        let should_continue = move || match stop_receiver.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => false,
            _ => true,
        };

        let join_handle = thread::Builder::new()
            .name("simulation thread".to_owned())
            .spawn(move || {
                let mut simulation = create();
                let mut update_timer = UpdateTimer::new("Sim Timer");

                simulation.setup(&mut input_buffer);

                while should_continue() {
                    let tick_time = update_timer.throttled_tick(throttle);
                    simulation.tick(&mut input_buffer, tick_time);
                }
            })?;

        Ok(Self {
            join_handle: Some(join_handle),
            stop_sender,
            output_buffer,
        })
    }

    /// The most recent version of the simulation's state.
    ///
    /// Repeated calls to this function can and will return different results
    /// as the simulation ticks in the background. The reference is valid until
    /// the next call to state().
    pub fn state(&mut self) -> &T {
        self.output_buffer.read()
    }
}

impl<T: Send> Drop for Worker<T> {
    /// Send the worker a stop signal then join the thread.
    fn drop(&mut self) {
        log::trace!("waiting for sim to drop");
        self.stop_sender
            .send(())
            .expect("unable to send a stop signal to the simulation thread");
        self.join_handle
            .take()
            .unwrap()
            .join()
            .expect("unable to join the simulation thread");
    }
}
