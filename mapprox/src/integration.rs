pub trait RiemannSum {
    fn left_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64;
    fn right_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64;
    fn trapezoid_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64 {
        (self.left_riemann_sum(a, b, n) + self.right_riemann_sum(a, b, n)) / 2.0
    }
    fn middle_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64;
    fn enhanced_middle_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64;
}

impl<F> RiemannSum for F
where
    F: Fn(f64) -> f64,
{
    fn left_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64 {
        let width = (b - a) / n as f64;
        let mut sum = 0.0;

        for i in 0..n {
            let x = a + i as f64 * width;
            sum += self(x) * width;
        }

        sum
    }

    fn right_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64 {
        let width = (b - a) / n as f64;
        let mut sum = 0.0;

        for i in 1..(n + 1) {
            let x = a + i as f64 * width;
            sum += self(x) * width;
        }

        sum
    }

    fn middle_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64 {
        let width = (b - a) / n as f64;
        let half_width = width / 2.0;
        let mut sum = 0.0;

        for i in 0..n {
            let x = a + i as f64 * width + half_width;
            sum += self(x) * width;
        }

        sum
    }

    fn enhanced_middle_riemann_sum(&self, a: f64, b: f64, n: usize) -> f64 {
        let g = |x| self((b - a) * x + a);
        (b - a) * g.middle_riemann_sum(0.0, 1.0, n)
    }
}
