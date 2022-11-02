mod solvers;
use crate::solvers::solvers::newtons_method;

fn main() {
    let root = newtons_method(&f, &f_prime, 0.5, 0.00001, 10000);
    println!("{}", root.unwrap());
}


fn f(x: f64) -> f64 {
    x+x.exp()-(2 as f64)
}

fn f_prime(x: f64) -> f64 {
    x.exp()+(1 as f64)
}