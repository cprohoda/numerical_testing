use std::{mem::{uninitialized, MaybeUninit}, os::unix::raw::pid_t};

use ndarray::prelude::*;
use ndarray_linalg::*;

pub fn main() {
    let x_n = [0.0, 1.0, 2.0];
    let y_n = [0.0, 1.0, 2.0];
    let z_n = [[0.0, 1.0, 3.0], [2.0, 4.0, 8.0], [5.0, 10.0, 16.0]];

    let polynomial = lagrange_interpolating_polynomial_3d(x_n, y_n, z_n);

    (0..20).for_each(|x| println!("{:?}", polynomial((x as f64)/10.0, 2.0)));
}

// pub fn lagrange_polynomial<const N: usize, F: Fn(f64) -> f64>(x: f64, l: [F; N]) -> f64 {
//     l.iter().enumerate().map(|(i, l_i)| {
//         l_i(x) * x.powi(i as i32)
//     }).sum()
// }

// pub fn lagrange_elementary_polynomials<const N: usize, F: Fn(f64) -> f64>(x_N: [f64; N]) -> [F; N] {
//     unsafe {
//         let mut elementary_polynomials: MaybeUninit<[F; N]> = std::mem::MaybeUninit::uninit();

//         // L_k(x) = Π_[j!=k](x-x_j) / Π_[j!=k](x_k-x_j)

//         elementary_polynomials.assume_init().iter_mut().enumerate().for_each(|(k, L_k)| {
//             std::ptr::write(L_k, |x| {
//                 x_N.iter().enumerate().filter(|(i, x)| {
//                     *i != k
//                 }).map(|(i, x_n)| {
//                     unimplemented!("");
//                     // figure out how to assemble a product operator in a loop
//                     // maybe macros
//                 }).product()
//             });
//         });

//         elementary_polynomials
//     }
// }

pub fn lagrange_elementary_polynomial_three(x_N: [f64; 3], y_N: [f64; 3]) -> impl Fn(f64) -> f64 {
    move |x| {
        y_N[0] * (x - x_N[2])/(x_N[0] - x_N[2]) * (x - x_N[1])/(x_N[0] - x_N[1]) +
        y_N[1] * (x - x_N[0])/(x_N[1] - x_N[0]) * (x - x_N[2])/(x_N[1] - x_N[2]) +
        y_N[2] * (x - x_N[0])/(x_N[2] - x_N[0]) * (x - x_N[1])/(x_N[2] - x_N[1])
    }
}

pub fn lagrange_interpolating_polynomial_3d(x_n: [f64; 3], y_n: [f64; 3], z_n: [[f64; 3]; 3]) -> impl Fn(f64, f64) -> f64 {
    move |x, y| {
        let li_0 = |x| (x - x_n[2])/(x_n[0] - x_n[2]) * (x - x_n[1])/(x_n[0] - x_n[1]);
        let li_1 = |x| (x - x_n[0])/(x_n[1] - x_n[0]) * (x - x_n[2])/(x_n[1] - x_n[2]);
        let li_2 = |x| (x - x_n[0])/(x_n[2] - x_n[0]) * (x - x_n[1])/(x_n[2] - x_n[1]);
        let lj_0 = |y| (y - y_n[2])/(y_n[0] - y_n[2]) * (y - y_n[1])/(y_n[0] - y_n[1]);
        let lj_1 = |y| (y - y_n[0])/(y_n[1] - y_n[0]) * (y - y_n[2])/(y_n[1] - y_n[2]);
        let lj_2 = |y| (y - y_n[0])/(y_n[2] - y_n[0]) * (y - y_n[1])/(y_n[2] - y_n[1]);

        li_0(x) * lj_0(y) * z_n[0][0] + li_1(x) * lj_0(y) * z_n[1][0] + li_2(x) * lj_0(y) * z_n[2][0] +
        li_0(x) * lj_1(y) * z_n[0][1] + li_1(x) * lj_1(y) * z_n[1][1] + li_2(x) * lj_1(y) * z_n[2][1] +
        li_0(x) * lj_2(y) * z_n[0][2] + li_1(x) * lj_2(y) * z_n[1][2] + li_2(x) * lj_2(y) * z_n[2][2]
    }
}

pub fn cubic_spline_interpolation(x_n: [f64; 3], y_n: [f64; 3]) -> impl Fn(f64) -> f64 {
    let h_i = 0.0; // h_i = x_ip1 - x_i;
    let n_i = 0.0; // n_i = y_ip1 - x_i;
    let a = vec![0.0]; // a_i = (b_ip1-b_i)/(3 * h_i)
    let b = vec![0.0]; // (n_ip1/h_ip1) - (n_i/h_i) = h_i*b_i/3 + 2*(h_ip1+h_i)*b_ip1/3 + h_i*b_ip2/3
    let c = 0.0; // n_i = a_i*h_i^3 + b_i*h_i^2 + c_i*h_i
    let d = 0.0; // d_i = y_i

    let g = 0.0; // a_i*(x-x_i)^3 + b_i*(x-x_i)^2 + c_i*(x-x_i) + d_i 
    move |x| {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lagrange_elementary_polynomial_three() {
        let x_N = [0.0, 1.0, 2.0];
        let y_N = [3.2, 3.1, 5.0];

        let polynomial = lagrange_elementary_polynomial_three(x_N, y_N);

        assert_eq!(polynomial(x_N[0]), y_N[0]);
        assert_eq!(polynomial(x_N[1]), y_N[1]);
        assert_eq!(polynomial(x_N[2]), y_N[2]);
    }
    
    #[test]
    fn test_lagrange_interpolating_polynomial_3d() {
        let x_n = [0.0, 1.0, 2.0];
        let y_n = [0.0, 1.0, 2.0];
        let z_n = [[0.0, 1.0, 3.0], [2.0, 4.0, 8.0], [5.0, 10.0, 16.0]];

        let polynomial = lagrange_interpolating_polynomial_3d(x_n, y_n, z_n);

        assert_eq!(polynomial(x_n[0], y_n[0]), z_n[0][0]);
        assert_eq!(polynomial(x_n[1], y_n[0]), z_n[1][0]);
        assert_eq!(polynomial(x_n[2], y_n[0]), z_n[2][0]);
        assert_eq!(polynomial(x_n[0], y_n[1]), z_n[0][1]);
        assert_eq!(polynomial(x_n[1], y_n[1]), z_n[1][1]);
        assert_eq!(polynomial(x_n[2], y_n[1]), z_n[2][1]);
        assert_eq!(polynomial(x_n[0], y_n[2]), z_n[0][2]);
        assert_eq!(polynomial(x_n[1], y_n[2]), z_n[1][2]);
        assert_eq!(polynomial(x_n[2], y_n[2]), z_n[2][2]);
    }
}
