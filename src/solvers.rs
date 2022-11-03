pub fn newtons_method(
    f: &dyn Fn(f64) -> f64,
    f_prime: &dyn Fn(f64) -> f64,
    x_0: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<f64, &'static str> {
    let mut x_k = x_0;
    for _ in 0..max_iterations {
        x_k = x_k - f(x_k) / f_prime(x_k);
        if f(x_k).abs() < tolerance {
            return Ok(x_k);
        }
    }
    Err("No root found in specified iterations")
}

pub fn secant_method(
    f: &dyn Fn(f64) -> f64,
    mut x_k: f64,
    mut x_kp1: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<f64, &'static str> {
    let mut holdover: f64;
    for _ in 0..max_iterations {
        holdover = x_kp1;
        x_kp1 = x_kp1 - (f(x_kp1) * (x_kp1 - x_k)) / (f(x_kp1) - f(x_k));
        if f(x_kp1).abs() < tolerance {
            return Ok(x_kp1);
        }
        x_k = holdover;
    }
    Err("No root found in specified iterations")
}
