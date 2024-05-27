use std::ops::{Add, Mul};

use nalgebra::{DMatrix, Matrix};

// Only for signed numbers, e.g. floats and ints
// TODO make generic over those types
struct Complex {
    r: f64,
    i: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            i: -self.i * rhs.i,
        }
    }
}

pub fn main() {
    dft2();
}

pub fn dft2(signal: [f64; 2]) -> [f64; 2] {
    let a = signal;
    [a[0] + a[1], a[0] - a[1]]
}

pub fn fft() {
    
}
