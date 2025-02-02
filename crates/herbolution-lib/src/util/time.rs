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

pub const TICK_INTERVAL: Duration = Duration::from_millis(1000 / 50);

pub struct TickTime {
    delta_time: DeltaTime,
    partial_tick_duration: Duration,
    tick_interval: Duration,
}

impl TickTime {
    pub fn update_clock(&mut self) -> Duration {
        let dt = self.delta_time.next();
        self.partial_tick_duration += dt;
        dt
    }

    pub fn should_tick(&mut self) -> bool {
        self.partial_tick_duration >= self.tick_interval
    }

    pub fn tick(&mut self) {
        self.partial_tick_duration -= self.tick_interval;
    }
}

impl Default for TickTime {
    fn default() -> Self {
        Self {
            delta_time: DeltaTime::default(),
            partial_tick_duration: Duration::ZERO,
            tick_interval: TICK_INTERVAL,
        }
    }
}
