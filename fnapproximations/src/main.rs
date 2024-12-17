use fnapproximations::derivative::SecantSlope;

fn main() {
    let f = |x: f64| x + 1.0 / x;

    let x = 2.;
    let a = f.auto_difference_quotient(x);
    println!("a derivative of f at {x}: {a}");
}
