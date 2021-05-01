use std::time::{Duration, Instant};

use super::UpdateTimer;

impl UpdateTimer {
    const UPDATE_INTERVAL: Duration = Duration::from_secs(5);

    pub fn new<Name: Into<String>>(name: Name) -> Self {
        Self {
            last_update: Instant::now(),
            last_checkpoint: Instant::now(),
            updates_since_checkpoint: 0,
            display_name: name.into(),
        }
    }

    /// Tick the timer and return the duration since the last update.
    ///
    /// Every so often, this method will write the average duration to the
    /// terminal.
    pub fn tick(&mut self) -> Duration {
        let tick_duration = self.last_update.elapsed();
        self.last_update = Instant::now();

        self.update_checkpoint();

        tick_duration
    }

    /// Tick the timer and return the duration since the last update.
    ///
    /// If the tick is less than the provided duration, then a `thread::sleep`
    /// is inserted to account for the difference.
    pub fn throttled_tick(&mut self, at_least: Duration) -> Duration {
        const MIN_SLEEP: Duration = Duration::from_millis(1);

        let raw_duration = self.last_update.elapsed();
        if at_least > raw_duration && at_least - raw_duration > MIN_SLEEP {
            spin_sleep::sleep(at_least - raw_duration);
        }

        self.tick()
    }

    fn update_checkpoint(&mut self) {
        let since_last_checkpoint = self.last_checkpoint.elapsed();
        if since_last_checkpoint >= Self::UPDATE_INTERVAL {
            let nanos = since_last_checkpoint.as_nanos() as f32;
            let nanos_per_update = nanos / self.updates_since_checkpoint as f32;
            let ms_per_update = nanos_per_update / 1e+6;
            let tps = f32::floor(1000.0 / ms_per_update) as i32;
            log::info!(
                "{} : {:.4} ms | {} tps",
                self.display_name,
                ms_per_update,
                tps
            );

            self.last_checkpoint = Instant::now();
            self.updates_since_checkpoint = 0;
        } else {
            self.updates_since_checkpoint += 1;
        }
    }
}
