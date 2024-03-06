use plotters::prelude::*;

pub fn main() {
    basic_plot();
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
