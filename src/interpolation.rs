pub fn main() {
    println!("{:?}", lagrange_interpolation(10.0, [1.0, 2.0, 3.0]));
}

pub fn lagrange_interpolation<const N: usize>(x: f64, a: [f64; N]) -> f64 {
    a.iter().enumerate().map(|(i, a_i)| {
        a_i * x.powi(i as i32)
    }).sum()
}
