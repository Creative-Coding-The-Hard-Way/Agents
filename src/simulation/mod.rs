//! This module provides types and functions for modelling an asynchronous,
//! stateful, simulation.

mod worker;

use std::{sync::mpsc::Sender, thread, time::Duration};
use triple_buffer::{Input, Output};

/// A simulation is a type with State. State is advanced in fixed time steps
/// and is presented via the triple-buffer's Input interface.
///
/// The big idea is that a simulation can be run in a separate thread at
/// whatever tick interval the specific implementation requires. Neither the
/// simulation nor the main thread need to block, instead the most recent state
/// is always made available via the triple-buffer.
///
/// Simulations are typically managed by a Worker instance.
pub trait Simulation {
    /// The portion of the simulation's state which will be sent between
    /// threads via the triple buffer.
    ///
    /// The state must be cloneable and defaultable so that it can be safely
    /// passed via the triple buffer.
    type SyncState: 'static + Clone + Send + Sized + Default;

    /// The minimum tick duration for this simulation. If an individual tick()
    /// invocation takes less than this amount of time, then the Worker will
    /// delay the next tick for the difference.
    const TICK_THROTTLE: Duration = Duration::from_millis(15);

    /// Initialize the simulation. Called once right before starting the main
    /// loop.
    fn setup(&mut self, sync: &mut Input<Self::SyncState>);

    /// Advance the simulation by one time step (fixed steps).
    fn tick(&mut self, sync: &mut Input<Self::SyncState>, duration: Duration);
}

/// A Simulation worker owns a thread which runs a simulation asynchronously.
pub struct Worker<TSim: Simulation> {
    join_handle: Option<thread::JoinHandle<()>>,
    stop_sender: Sender<()>,
    output_buffer: Output<TSim::SyncState>,
}
