use std::time::{Duration, Instant};

pub struct Fps {
    previous_time: Instant,
    acc: u32,
    last: u32,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            previous_time: Instant::now(),
            acc: 0,
            last: 0,
        }
    }

    pub fn update(&mut self) {
        self.acc += 1;

        let now = Instant::now();
        if now - self.previous_time >= Duration::SECOND {
            self.last = self.acc;
            self.acc = 0;
            self.previous_time = now;
        }
    }

    pub fn get(&self) -> u32 {
        self.last
    }
}