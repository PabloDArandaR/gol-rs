use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Scheduler {
    refresh_rate: Duration,
    last_timestamp: Option<Instant>,
}

impl Scheduler {
    pub fn new(refresh_rate: Duration) -> Self {
        Self {
            refresh_rate: refresh_rate,
            last_timestamp: None,
        }
    }

    pub fn wait_for_next(&mut self) -> bool {
        if self.last_timestamp.is_none() {
            return false;
        }

        sleep(self.last_timestamp.unwrap() + self.refresh_rate - Instant::now());
        self.last_timestamp = Some(Instant::now());
        true
    }

    pub fn start(&mut self) {
        self.last_timestamp = Some(Instant::now());
    }
}
