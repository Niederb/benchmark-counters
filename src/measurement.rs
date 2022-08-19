pub struct Measurement {
    min: u128,
    max: u128,
    total: u128,
    measurement_count: u128,
}

impl Measurement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn measuring_point(&mut self, p: u128) {
        if self.measurement_count == 0 {
            self.min = p;
            self.max = p;
            self.total = p;
        } else {
            self.min = self.min.min(p);
            self.max = self.max.max(p);
            self.total += p;
        }
        self.measurement_count += 1;
    }

    pub fn reset(&mut self) {
        self.min = 0;
        self.max = 0;
        self.total = 0;
        self.measurement_count = 0;
    }

    pub fn avg(&self) -> f32 {
        if self.measurement_count == 0 {
            return 0.0;
        }
        self.total as f32 / self.measurement_count as f32
    }

    pub fn min_max_avg(&self) -> (u128, u128, f32) {
        (self.min, self.max, self.avg())
    }
}

impl Default for Measurement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let m = Measurement::new();
        assert_eq!((0, 0, 0.0), m.min_max_avg());
    }

    #[test]
    fn test_single() {
        let mut m = Measurement::new();
        m.measuring_point(10);
        assert_eq!((10, 10, 10.0), m.min_max_avg());
    }

    #[test]
    fn test_double_with_reset() {
        let mut m = Measurement::new();
        m.measuring_point(10);
        m.measuring_point(20);
        assert_eq!((10, 20, 15.0), m.min_max_avg());
        m.reset();
        assert_eq!((0, 0, 0.0), m.min_max_avg());
    }
}
