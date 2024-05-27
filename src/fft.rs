use std::ops::{Add, Mul, Sub};

// Only for signed numbers, e.g. floats and ints
// TODO make generic over those types
#[derive(Debug)]
struct Complex {
    r: f64,
    i: f64,
}

impl Complex {
    fn new(r: f64, i: f64) -> Self {
        Self {
            r,
            i,
        }
    }

    fn r(r: f64) -> Self {
        Self {
            r,
            i: 0.0,
        }
    }

    fn i(i: f64) -> Self {
        Self {
            r: 0.0,
            i,
        }
    }
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

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i,
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
    println!("dft2([1.0, 3.0]): {:?}", dft2([1.0, 3.0]));
    println!
        ("dft4([1.0, 0.0, 3.0, 0.0]): {:?}",
        dft4([
            Complex::r(1.0),
            Complex::r(0.0),
            Complex::r(3.0),
            Complex::r(0.0),
        ]),
    );
}

pub fn dft2(signal: [f64; 2]) -> [f64; 2] {
    let a = signal;
    [a[0] + a[1], a[0] - a[1]]
}

pub fn dft4(signal: [Complex; 4]) -> [Complex; 4] {
    let a = signal;
    let i = Complex{ r: 0.0, i: 1.0 };

    [
        a[0] + a[1] + a[2] + a[3],
        a[0] - i * a[1] - a[2] + a[3],
        a[0] - a[1] + a[2] - a[3],
        a[0] + i * a[1] - a[2] - i * a[3],
    ]
}

pub fn fft() {
    
}
