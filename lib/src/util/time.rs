use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use time::ext::InstantExt;
use time::Duration;

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
        let dt = now.signed_duration_since(self.instant);
        self.instant = now;
        dt
    }
}

#[derive(Debug)]
pub struct TickTime {
    interval: Duration,
    acc: Duration,
    rate: u64,
}

impl TickTime {
    pub fn new(ticks_per_second: u64) -> Self {
        Self {
            interval: Duration::milliseconds(1000 / ticks_per_second as i64),
            acc: Duration::ZERO,
            rate: ticks_per_second,
        }
    }

    pub fn set_rate(&mut self, ticks_per_second: u64) {
        self.rate = ticks_per_second;
        self.interval = Duration::milliseconds(1000 / ticks_per_second as i64);
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

pub struct ProgressiveMeasurement {
    count: AtomicU64,
    acc: AtomicU64,
}

impl ProgressiveMeasurement {
    pub const fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
            acc: AtomicU64::new(0),
        }
    }

    pub fn start_measuring(&self) -> Stopwatch<'_> {
        Stopwatch {
            stopwatch: self,
            start: Instant::now(),
        }
    }

    pub fn average(&self) -> Duration {
        let count = self.count.load(Ordering::Relaxed);
        if count == 0 {
            Duration::ZERO
        } else {
            Duration::nanoseconds((self.acc.load(Ordering::Relaxed) / count) as i64)
        }
    }
}

pub struct Stopwatch<'a> {
    stopwatch: &'a ProgressiveMeasurement,
    start: Instant,
}

impl Stopwatch<'_> {
    pub fn stop(self) {
        let elapsed = self.start.elapsed();
        let nanos = elapsed.as_nanos() as u64;

        self.stopwatch
            .count
            .fetch_add(1, Ordering::Relaxed);
        let acc = self
            .stopwatch
            .acc
            .fetch_add(nanos, Ordering::Relaxed);

        if u64::MAX - acc < nanos * 16 {
            self.stopwatch.count.store(0, Ordering::Relaxed);
            self.stopwatch.acc.store(0, Ordering::Relaxed);
        }
    }
}
