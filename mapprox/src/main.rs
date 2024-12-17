use mapprox::derivative::Differentiable;

fn main() {
    let f0 = |x| x * x * x;
    let x = 1.;

    let f1 = f0.derivative(); // f(x) = 3x^2
    let f2 = f1.derivative(); // f(x) = 6x
    let f3 = f2.derivative(); // f(x) = 6
    let f4 = f3.derivative(); // f(x) = 0
    let f5 = f4.derivative(); // f(x) = 0
    let f6 = f5.derivative(); // f(x) = 0
    let f7 = f6.derivative(); // f(x) = 0

    println!("f0 at {x}: {}", f0(x));
    println!("f1 at {x}: {}", f1(x));
    println!("f2 at {x}: {}", f2(x));
    println!("f3 at {x}: {}", f3(x));
    println!("f4 at {x}: {}", f4(x));
    println!("f5 at {x}: {}", f5(x));
    println!("f6 at {x}: {}", f6(x));
    println!("f7 at {x}: {}", f7(x));
}
