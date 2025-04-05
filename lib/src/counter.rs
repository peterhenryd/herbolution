use std::time::Duration;
use heapless::HistoryBuffer;

pub struct IntervalCounter {
    interval: Duration,
    acc: Duration,
    current: u64,
    last: u64,
    buffer: HistoryBuffer<u64, 16>,
    avg: u64,
}

impl IntervalCounter {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            acc: Duration::ZERO,
            current: 0,
            last: 0,
            buffer: HistoryBuffer::new(),
            avg: 0,
        }
    }

    pub fn add_delta(&mut self, dt: Duration) {
        self.acc += dt;
    }

    pub fn update(&mut self, dt: Duration) {
        self.acc += dt;
        self.current += 1;

        if self.acc >= self.interval {
            self.last = self.current;
            self.current = 0;
            self.acc -= self.interval;
            self.buffer.write(self.last);
            self.avg = self.buffer.iter().sum::<u64>() / self.buffer.len() as u64;
        }
    }

    #[inline]
    pub fn last(&self) -> u64 {
        self.last
    }

    #[inline]
    pub fn avg(&self) -> u64 {
        self.avg
    }
}