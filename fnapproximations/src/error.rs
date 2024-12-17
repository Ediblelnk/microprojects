pub fn percent_difference(a: f64, b: f64) -> f64 {
    (2.0 * (a - b) / (a + b)).abs()
}
