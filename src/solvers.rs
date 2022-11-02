pub mod solvers {
    pub fn newtons_method(f: &dyn Fn(f64) -> f64, f_prime: &dyn Fn(f64) -> f64, x_0: f64, tolerance: f64, max_iterations: usize) -> Result<f64, &'static str> {
        let mut x_k = x_0;
        for _ in 0..max_iterations {
            x_k = x_k - f(x_k)/f_prime(x_k);
            if f(x_k).abs() < tolerance {
                return Ok(x_k)
            }
        }
        Err("No root found in specified iterations")
    }
}