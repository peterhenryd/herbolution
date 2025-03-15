#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Counter {
    current: u64,
    interval: u64,
}

impl Counter {
    pub fn new(interval: u64) -> Self {
        Self {
            current: 0,
            interval,
        }
    }

    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
    }

    pub fn check(&mut self) -> bool {
        self.current += 1;

        if self.current >= self.interval {
            self.current = 0;
            true
        } else {
            false
        }
    }
}
