mod solvers;

fn main() {
    let root1 = solvers::newtons_method(&f_1, &f_1_prime, 0.5, 0.00001, 10000);
    println!("{}", root1.unwrap());
    let root2 = solvers::secant_method(&f_1, 0.0, 1.0, 0.00001, 10000);
    println!("{}", root2.unwrap());
    let root3 = solvers::bisection_method(&f_1, 0.0, 1.0, 0.00001, 10000);
    println!("{}", root3.unwrap());
    let root4 = solvers::fixed_point_method(&g_1_b, 0.1, 0.00001, 10000);
    println!("{}", root4.unwrap());
}

fn f_1(x: f64) -> f64 {
    x + x.exp() - 2.0
}

fn f_1_prime(x: f64) -> f64 {
    x.exp() + 1.0
}

fn g_1_a(x: f64) -> f64 {
    2.0 - x.exp()
}

fn g_1_b(x: f64) -> f64 {
    (2.0 - x).ln()
}

fn f_2(x: f64) -> f64 {
    2.0 * x - x.tan()
}

fn f_2_prime(x: f64) -> f64 {
    2.0 - 1.0 / (x.cos().powf(2.0))
}

fn g_2_a(x: f64) -> f64 {
    0.5 * x.tan()
}

fn g_2_b(x: f64) -> f64 {
    (2.0 * x).atan()
}

fn f_3(a: f64, x: f64) -> f64 {
    -a * x.powf(2.0) + 2.0 * a
}

fn g_3(a: f64, x: f64) -> f64 {
    -a * x.powf(2.0) + 2.0 * a + x
}
