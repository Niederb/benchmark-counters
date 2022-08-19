use std::time::{Duration, Instant};

pub struct IntervalChecker {
    min: Duration,
    max: Duration,
    last_interval: Option<Instant>,
    interval_counter: usize,
    errors: usize,
}

impl IntervalChecker {
    pub fn new(min: Duration, max: Duration) -> IntervalChecker {
        IntervalChecker {
            min,
            max,
            last_interval: None,
            interval_counter: 0,
            errors: 0,
        }
    }

    pub fn tick<F>(&mut self, callback: F) -> bool
    where
        F: FnOnce(&IntervalChecker, &Duration),
    {
        let mut success = true;
        if let Some(last_interval) = self.last_interval {
            let elapsed = last_interval.elapsed();
            if elapsed > self.max || elapsed < self.min {
                self.errors += 1;
                success = false;
                callback(self, &elapsed);
            }
        }
        self.last_interval = Some(Instant::now());
        self.interval_counter += 1;
        success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_interval() {
        let mut c = IntervalChecker::new(Duration::from_millis(100), Duration::from_millis(1000));
        c.tick(|_, _| panic!());
        thread::sleep(Duration::from_millis(200));
        c.tick(|_, _| panic!());
        thread::sleep(Duration::from_millis(20));
        let mut called = false;
        c.tick(|checker, elapsed| {
            called = true;
            assert!(elapsed.as_millis() < 100 && elapsed.as_millis() >= 20);
            assert_eq!(2, checker.interval_counter);
            assert_eq!(1, checker.errors);
        });
        assert!(called);
    }
}
