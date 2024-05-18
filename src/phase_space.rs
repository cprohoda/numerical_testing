use std::f64::consts::PI;

use plotters::{backend::BitMapBackend, chart::{ChartBuilder, LabelAreaPosition}, drawing::IntoDrawingArea, series::LineSeries, style::{BLACK, WHITE}};

pub fn main() {
    phase_space_plot();
}

fn pendulum(mut theta: f64, mut theta_dot: f64) -> (f64, f64) {
    let delta_t = 1.0/100.0;

    let theta_double_dot = get_theta_double_dot(theta, theta_dot);

    theta += theta_dot * delta_t;
    theta_dot += theta_double_dot * delta_t;

    (theta, theta_dot)
}

fn get_theta_double_dot(theta: f64, theta_dot: f64) -> f64 {
    let g = 9.8;
    let l = 2.0;
    let mu = 0.1;

    return -mu * theta_dot - (g / l) * f64::sin(theta);
}

fn phase_space_plot() {
    let root_area = BitMapBackend::new("target/pendulum_phase.png", (1000, 1000)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Pendulum Phase Space", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(
        (0..100).scan(
            (3.0, 0.0),
            |(theta, theta_dot), _x| Some(pendulum(*theta, *theta_dot)),
        ),
        &BLACK,
    )).unwrap();
}
