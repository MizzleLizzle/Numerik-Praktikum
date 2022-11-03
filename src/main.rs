mod solvers;

fn main() {
    let root1 = solvers::newtons_method(&f, &f_prime, 0.5, 0.00001, 10000);
    println!("{}", root1.unwrap());
    let root2 = solvers::secant_method(&f, 0.0, 1.0, 0.00001, 10000);
    println!("{}", root2.unwrap());
}

fn f(x: f64) -> f64 {
    x + x.exp() - (2 as f64)
}

fn f_prime(x: f64) -> f64 {
    x.exp() + (1 as f64)
}
