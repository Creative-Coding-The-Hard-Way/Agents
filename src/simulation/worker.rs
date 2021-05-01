use super::{Simulation, Worker};

use crate::app::UpdateTimer;
use anyhow::Result;
use std::{
    sync::mpsc::{self, TryRecvError},
    thread,
};
use triple_buffer::TripleBuffer;

impl<TSim: 'static + Simulation> Worker<TSim> {
    /// Create a new simulation worker instance.
    pub fn new(create: fn() -> TSim) -> Result<Self> {
        // a channel is used to signal when the worker should terminate
        let (stop_sender, stop_receiver) = mpsc::channel();

        // a triple-buffer with initial data set to the default
        let (mut input_buffer, output_buffer) =
            TripleBuffer::new(TSim::SyncState::default()).split();

        let should_continue = move || match stop_receiver.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => false,
            _ => true,
        };

        let join_handle = thread::Builder::new()
            .name("simulation thread".to_owned())
            .spawn(move || {
                // Create the simulation
                let mut simulation = create();
                let mut update_timer = UpdateTimer::new("Sim Timer");

                simulation.setup(&mut input_buffer);

                while should_continue() {
                    let tick_time =
                        update_timer.throttled_tick(TSim::TICK_THROTTLE);
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
    pub fn state(&mut self) -> &TSim::SyncState {
        self.output_buffer.read()
    }
}

impl<TSim: Simulation> Drop for Worker<TSim> {
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
