const H: f64 = 0.001;

pub trait Differentiable {
    fn difference_quotient(&self, h: f64) -> impl Fn(f64) -> f64;
    fn symmetric_difference_quotient(&self, h: f64) -> impl Fn(f64) -> f64;
    fn derivative(&self) -> impl Fn(f64) -> f64 {
        |x| self.symmetric_difference_quotient(H)(x)
    }

    fn second_symmetric_derivative(&self, h: f64) -> impl Fn(f64) -> f64;
    fn second_derivative(&self) -> impl Fn(f64) -> f64 {
        |x| self.second_symmetric_derivative(H)(x)
    }
}

impl<F> Differentiable for F
where
    F: Fn(f64) -> f64,
{
    fn difference_quotient(&self, h: f64) -> impl Fn(f64) -> f64 {
        move |x| (self(x + h) - self(x)) / h
    }

    fn symmetric_difference_quotient(&self, h: f64) -> impl Fn(f64) -> f64 {
        move |x| (self(x + h) - self(x - h)) / (2. * h)
    }

    fn second_symmetric_derivative(&self, h: f64) -> impl Fn(f64) -> f64 {
        move |x| (self(x + h) - 2. * self(x) + self(x - h)) / (h * h)
    }
}
