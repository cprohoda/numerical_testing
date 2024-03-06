use plotters::prelude::*;

pub fn main() {
    // basic_plot();
    interpolation_2d_plot();
}

pub fn basic_plot() {
    let root_area = BitMapBackend::new("target/test.png", (1000, 1000)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .build_cartesian_2d(-3.14..3.14, -3.14..3.14)
        .unwrap();

    chart.draw_series(LineSeries::new(
        (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
        &BLACK,
    )).unwrap();
}

pub fn interpolation_2d_plot() {
    use crate::interpolation::lagrange_elementary_polynomial_three;

    let root_area = BitMapBackend::new("target/interpolation_2d.png", (1000, 1000)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)
        .unwrap();

    let polynomial = lagrange_elementary_polynomial_three([0.0, 3.0, 5.0], [3.2, 3.1, 5.0]);

    chart.draw_series(LineSeries::new(
        (0..100).map(|x| x as f64 / 10.0).map(|x| (x, polynomial(x))),
        &BLACK,
    )).unwrap();
}
