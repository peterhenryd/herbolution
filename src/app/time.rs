use std::time::{Duration, Instant};

pub struct DeltaTime(Instant);

impl DeltaTime {
    pub fn next(&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.0;
        self.0 = now;
        dt
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}