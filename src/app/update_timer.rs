use std::time::{Duration, Instant};

use super::UpdateTimer;

impl Default for UpdateTimer {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            last_checkpoint: Instant::now(),
            updates_since_checkpoint: 0,
        }
    }
}

impl UpdateTimer {
    const UPDATE_INTERVAL: Duration = Duration::from_secs(5);

    /// Tick the timer and return the duration since the last update.
    ///
    /// Every so often, this method will write the average duration to the
    /// terminal.
    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let since_last_update = now - self.last_update;
        self.last_update = now;

        let since_last_checkpoint = now - self.last_checkpoint;
        if since_last_checkpoint >= Self::UPDATE_INTERVAL {
            let nanos = since_last_checkpoint.as_nanos() as f32;
            let nanos_per_update = nanos / self.updates_since_checkpoint as f32;
            let ms_per_update = nanos_per_update / 1e+6;
            let fps = f32::floor(1000.0 / ms_per_update) as i32;
            log::info!(
                "Avg Update Duration: {:.4} ms | {} fps",
                ms_per_update,
                fps
            );

            self.last_checkpoint = now;
            self.updates_since_checkpoint = 0;
        } else {
            self.updates_since_checkpoint += 1;
        }

        since_last_update
    }
}
