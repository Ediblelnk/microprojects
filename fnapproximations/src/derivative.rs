use crate::error::percent_difference;

pub trait SecantSlope {
    fn difference_quotient(&self, x: f64, h: f64) -> f64;
    fn symmetric_difference_quotient(&self, x: f64, h: f64) -> f64;
    fn auto_difference_quotient(&self, x: f64) -> f64 {
        let mut h = 0.5;
        let shrink_factor = 0.5;
        let mut last_slope = self.symmetric_difference_quotient(x, h / shrink_factor);
        let mut last_difference = 1.0;

        loop {
            let new_slope = self.symmetric_difference_quotient(x, h);
            let new_difference = percent_difference(last_slope, new_slope);

            if new_difference >= last_difference {
                break last_slope;
            }

            last_slope = new_slope;
            last_difference = new_difference;

            h *= shrink_factor;
        }
    }
}

impl<F> SecantSlope for F
where
    F: Fn(f64) -> f64,
{
    fn difference_quotient(&self, x: f64, h: f64) -> f64 {
        (self(x + h) - self(x)) / h
    }

    fn symmetric_difference_quotient(&self, x: f64, h: f64) -> f64 {
        (self(x + h) - self(x - h)) / (2. * h)
    }
}
