use std::mem::{MaybeUninit, uninitialized};

pub fn main() {
    println!("{:?}", lagrange_polynomial(10.0, [1.0, 2.0, 3.0]));
}

pub fn lagrange_polynomial<const N: usize, F: Fn(f64) -> f64>(x: f64, l: [F; N]) -> f64 {
    l.iter().enumerate().map(|(i, l_i)| {
        l_i(x) * x.powi(i as i32)
    }).sum()
}

pub fn lagrange_elementary_polynomials<const N: usize, F: Fn(f64) -> f64>(x_N: [f64; N]) -> [F; N] {
    unsafe {
        let mut elementary_polynomials: MaybeUninit<[F; N]> = std::mem::MaybeUninit::uninit();

        // L_k(x) = Π_[j!=k](x-x_j) / Π_[j!=k](x_k-x_j)

        elementary_polynomials.assume_init().iter_mut().enumerate().for_each(|(k, L_k)| {
            std::ptr::write(L_k, |x| {
                x_N.iter().enumerate().filter(|(i, x)| {
                    *i != k
                }).map(|(i, x_n)| {
                    unimplemented!("");
                    // figure out how to assemble a product operator in a loop
                    // maybe macros
                }).product()
            });
        });

        elementary_polynomials
    }
}
