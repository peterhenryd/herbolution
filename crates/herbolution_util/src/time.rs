use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct DeltaTime {
    instant: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self { instant: Instant::now() }
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
    acc: Duration,
    rate: u64,
}

impl TickTime {
    pub fn new(ticks_per_second: u64) -> Self {
        Self {
            interval: Duration::from_millis(1000 / ticks_per_second),
            acc: Duration::ZERO,
            rate: ticks_per_second,
        }
    }

    pub fn set_rate(&mut self, ticks_per_second: u64) {
        self.rate = ticks_per_second;
        self.interval = Duration::from_millis(1000 / ticks_per_second);
    }

    #[inline]
    pub fn increment(&mut self, dt: Duration) {
        self.acc += dt;
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        self.acc >= self.interval
    }

    #[inline]
    pub fn reduce(&mut self) {
        self.acc -= self.interval;
    }
}

#[derive(Debug)]
pub struct IntervalCounter {
    interval: Duration,
    acc: Duration,
    current: u64,
    last: u64,
}

impl IntervalCounter {
    pub fn new(interval: Duration) -> Self {
        Self {
            acc: Duration::ZERO,
            interval,
            current: 0,
            last: 0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.acc += dt;
        self.current += 1;

        if self.acc >= self.interval {
            self.last = self.current;
            self.current = 0;
            self.acc -= self.interval;
        }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.last
    }
}
