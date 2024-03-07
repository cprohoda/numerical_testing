use std::iter::repeat;

use plotters::prelude::*;

pub fn main() {
    // basic_plot();
    // interpolation_2d_plot();
    interpolation_3d_plot();
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
        .caption("2d Interpolation", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let polynomial = lagrange_elementary_polynomial_three(
        [0.0, 3.0, 5.0],
        [3.2, 3.1, 5.0],
    );

    chart.draw_series(LineSeries::new(
        (0..100).map(|x| x as f64 / 10.0).map(|x| (x, polynomial(x))),
        &BLACK,
    )).unwrap();
}

pub fn interpolation_3d_plot() {
    use crate::interpolation::lagrange_interpolating_polynomial_3d;

    let root = BitMapBackend::new("target/interpolation_3d.png", (1000, 1000)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(50)
        .caption("3d Interpolation", ("Arial", 20))
        .build_cartesian_3d(0.0..2.5, 0.0..2.5, 0.0..30.0)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    let polynomial = lagrange_interpolating_polynomial_3d(
        [0.0, 1.0, 2.0], 
        [0.0, 1.0, 2.0],
        [[0.0, 1.0, 3.0], [2.0, 4.0, 8.0], [5.0, 10.0, 16.0]],
    );

    chart.draw_series(
        (0..250)
            .map(|x| repeat(x).zip(0..250))
            .flatten()
            .map(|(x, y)| {
                let x_current = x as f64;
                let x_next = (x + 1)  as f64;
                let y_current = y as f64;
                let y_next = (y + 1) as f64;

                Polygon::new(
                    vec![
                        (x_current, y_current, polynomial(x_current,y_current)),
                        (x_next, y_current, polynomial(x_next, y_current)),
                        (x_current, y_next, polynomial(x_current, y_next)),
                        (x_next, y_next, polynomial(x_next, y_next)),
                    ],
                    &BLACK.mix(0.3),
                )
            })
    ).unwrap();
}
