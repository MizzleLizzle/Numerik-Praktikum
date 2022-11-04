mod solvers;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_plot(&f_1)?;
    let series = solvers::newtons_method_error_series(&f_1, &f_1_prime, 1.0, 0.00001, 10000);
    println!("{:?}", series.unwrap());
    let series2 = solvers::secant_method_error_series(&f_1, 0.0, 1.0, 0.00001, 10000);
    println!("{:?}", series2.unwrap());
    let root3 = solvers::bisection_method(&f_1, 0.0, 1.0, 0.00001, 10000);
    println!("{}", root3.unwrap());
    let root4 = solvers::fixed_point_method(&g_1_b, 0.1, 0.00001, 10000);
    println!("{}", root4.unwrap());
    Ok(())
}

fn build_plot(
    f: &dyn Fn(f64) -> f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("{name}.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("name", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-2f64..2f64, -2f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f64 / 50.0).map(|x| (x, f(x))),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
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
