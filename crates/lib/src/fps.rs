use std::time::Duration;

// TODO: implement weighted average
pub struct Fps {
    sec_accumulator: Duration,
    current: u64,
    last: u64,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            sec_accumulator: Duration::ZERO,
            current: 0,
            last: 0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.sec_accumulator += dt;
        self.current += 1;

        if self.sec_accumulator >= Duration::SECOND {
            self.last = self.current;
            self.current = 0;
            self.sec_accumulator -= Duration::SECOND;
        }
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.last
    }
}