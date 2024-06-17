use std::{f64::consts::PI, iter::Sum, ops::{Add, Div, Mul, Neg, Sub}};

// Only for signed numbers, e.g. floats and ints
// TODO make generic over those types
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    r: f64,
    i: f64,
}

impl Complex {
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

    fn exp(self) -> Self {
        let factor = self.r.exp();

        Self {
            r: factor * self.i.cos(),
            i: factor * self.i.sin(),
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

impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self {r: 0.0, i: 0.0}, |acc, curr| {
            Self {
                r: acc.r + curr.r,
                i: acc.i + curr.i,
            }
        })
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
        // (a + ib) * (c + id) = ac + iad + ibc - bd
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            i: self.i * rhs,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r / rhs,
            i: self.i / rhs,
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
    println!
        ("dft([1.0, 0.0, 3.0, 0.0]): {:?}",
        dft::<4>([
            Complex::r(1.0),
            Complex::r(0.0),
            Complex::r(3.0),
            Complex::r(0.0),
        ]),
    );
    println!
        ("fft4([1.0, 0.0, 3.0, 0.0]): {:?}",
        fft4([
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
    let i = Complex::i(1.0);

    [
        a[0] + a[1] + a[2] + a[3],
        a[0] - i * a[1] - a[2] + i * a[3],
        a[0] - a[1] + a[2] - a[3],
        a[0] + i * a[1] - a[2] - i * a[3],
    ]
}

pub fn dft<const N: usize>(signal: [Complex; N]) -> [Complex; N] {
    let i = Complex::i(1.0);
    let w = -i * 2.0 * PI / (N as f64);

    core::array::from_fn(|k| {
        (0..N).map(|n| {
            (w * (n as f64) * (k as f64)).exp() * signal[n]
        }).sum()
    })
}

pub fn fft4(signal: [Complex; 4]) -> [Complex; 4] {
    let i = Complex::i(1.0);

    let subexp: [Complex; 4] = core::array::from_fn(|n| {
        let b = (n as f64 / 2.0).floor() as usize;
        let sign = -((n % 2) as f64 - 1.0);
        signal[b] + (signal[b+2] * sign)
    });

    core::array::from_fn(|n| {
        let joiner = (0..n).fold(Complex::r(1.0), |curr, _| curr * -i);
        subexp[n % 2] + joiner * subexp[n / 2 + 2]
    })
}
