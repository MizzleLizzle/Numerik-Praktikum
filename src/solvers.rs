pub fn newtons_method(
    f: &dyn Fn(f64) -> f64,
    f_prime: &dyn Fn(f64) -> f64,
    mut x: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<f64, &'static str> {
    for _ in 0..max_iterations {
        x = x - f(x) / f_prime(x);
        if f(x).abs() < tolerance {
            return Ok(x);
        }
    }
    Err("No root found in specified iterations")
}

pub fn newtons_method_error_series(
    f: &dyn Fn(f64) -> f64,
    f_prime: &dyn Fn(f64) -> f64,
    mut x: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<Vec<f64>, &'static str> {
    let mut errors: Vec<f64> = vec![x];
    for _ in 0..max_iterations {
        x = x - f(x) / f_prime(x);
        errors.push(x);
        if f(x).abs() < tolerance {
            errors = errors.iter().map(|y| (x - y).abs()).collect();
            return Ok(errors);
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

pub fn secant_method_error_series(
    f: &dyn Fn(f64) -> f64,
    mut x_k: f64,
    mut x_kp1: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<Vec<f64>, &'static str> {
    let mut holdover: f64;
    let mut errors = vec![x_k, x_kp1];
    for _ in 0..max_iterations {
        holdover = x_kp1;
        x_kp1 = x_kp1 - (f(x_kp1) * (x_kp1 - x_k)) / (f(x_kp1) - f(x_k));
        errors.push(x_kp1);
        if f(x_kp1).abs() < tolerance {
            errors = errors.iter().map(|y| (x_kp1 - y).abs()).collect();
            return Ok(errors);
        }
        x_k = holdover;
    }
    Err("No root found in specified iterations")
}

pub fn bisection_method(
    f: &dyn Fn(f64) -> f64,
    mut a: f64,
    mut b: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<f64, &'static str> {
    if f(a).signum() == f(b).signum() {
        return Err("Chosen interval doesn't guarantee sign change/root");
    }
    let mut x: f64;
    for _ in 0..max_iterations {
        x = (a + b) / 2.0;
        if f(x).abs() < tolerance {
            return Ok(x);
        }
        match f(x).signum() == f(a).signum() {
            true => a = x,
            _ => b = x,
        }
    }
    Err("No root found in specified iterations")
}

pub fn fixed_point_method(
    g: &dyn Fn(f64) -> f64,
    mut x: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<f64, &'static str> {
    for _ in 0..max_iterations {
        x = g(x);
        if (x - g(x)).abs() < tolerance {
            return Ok(x);
        }
    }
    Err("No root found in specified iterations")
}
