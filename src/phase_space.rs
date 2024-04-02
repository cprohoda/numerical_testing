use std::f64::consts::PI;

pub fn main() {
    pendulum();
}

fn pendulum() {
    let steps = 10000;
    let delta_t = 1.0/100.0;

    let mut theta = PI/4.0;
    let mut theta_dot = 0.0;

    println!("theta: {:?}      theta_dot: {:?}", theta, theta_dot);

    for step in 0..steps {
        let t = step as f64 * delta_t;

        let theta_double_dot = get_theta_double_dot(theta, theta_dot);

        theta += theta_dot * delta_t;
        theta_dot += theta_double_dot * delta_t;

        println!("theta: {:?}      theta_dot: {:?}", theta, theta_dot);
    }
}

fn get_theta_double_dot(theta: f64, theta_dot: f64) -> f64 {
    let g = 9.8;
    let l = 2.0;
    let mu = 0.1;

    return -mu * theta_dot - (g / l) * f64::sin(theta);
}
