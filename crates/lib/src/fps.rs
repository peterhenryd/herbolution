use std::time::Duration;

#[derive(Debug)]
pub struct IntervalCounter {
    acc: Duration,
    interval: Duration,
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