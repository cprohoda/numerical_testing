pub fn main() {
    acceptable_error();
}

pub fn acceptable_error() {
    let f = |x: f32| x.powi(2) + x - 1.0;
    let acceptable_error: f32 = 1.0e-10;

    let mut x = 0.0;
    let mut candidate_x = x.clone();
    let mut error = f(x);
    let mut candidate_error = error.clone();
    let base_dx = 1.0e-1;
    let mut dx = base_dx.clone();
    let mut i = 0;
    let mut slope = 0.0;

    while error.abs() > acceptable_error.abs() && i < 100 {
        candidate_x = x + dx;
        candidate_error = f(candidate_x);
        slope = (candidate_error - error) / (candidate_x - x);
        x -= candidate_error/slope;
        error = f(x);
        i += 1;
        println!("x = {:?}, error = {:?}", x, error);
    }

    println!("Final result:\nx = {:?}\nerror = {:?}", x, error);
}
