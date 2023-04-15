use std::time::{Duration, Instant};

pub struct StopWatch {
    start_time: Option<Instant>,
    elapsed: Duration,
}

impl StopWatch {
    pub fn new() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::default(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.elapsed = self.elapsed();
        self.start_time = None;
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::default();
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(t) => t.elapsed() + self.elapsed,
            None => self.elapsed,
        }
    }

    pub fn is_running(&self) -> bool {
        self.start_time.is_some()
    }
}

impl Default for StopWatch {
    fn default() -> Self {
        Self::new()
    }
}
