use std::time::{Duration, Instant};

pub struct DeltaTime(Instant);

impl DeltaTime {
    pub fn next_delta(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.0;
        self.0 = now;
        delta
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}