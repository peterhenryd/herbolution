use std::time::{Duration, Instant};

pub struct DeltaTime {
    instant: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }

    pub fn next(&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.instant;
        self.instant = now;
        dt
    }
}

pub struct TickTime {
    interval: Duration,
    accumulator: Duration,
    ticks_per_second: u64,
}

impl TickTime {
    pub fn new(ticks_per_second: u64) -> Self {
        Self {
            interval: Duration::from_millis(1000 / ticks_per_second),
            accumulator: Duration::ZERO,
            ticks_per_second,
        }
    }

    pub fn set_ticks_per_second(&mut self, ticks_per_second: u64) {
        self.ticks_per_second = ticks_per_second;
        self.interval = Duration::from_millis(1000 / ticks_per_second);
    }

    pub fn increment(&mut self, dt: Duration) {
        self.accumulator += dt;
    }

    pub fn is_ready(&self) -> bool {
        self.accumulator >= self.interval
    }

    pub fn reduce(&mut self) {
        self.accumulator -= self.interval;
    }
}