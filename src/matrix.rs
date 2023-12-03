pub fn main() {
    println!("{:?}", Matrix::<2, 2>::new(0.));
    println!("{:?}", Matrix::<2, 2>::new_matrix([[1., 1.], [2., 3.]]));
}

#[derive(Debug)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[f64; M]; N],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new(init: f64) -> Self {
        Self {
            data: [[init; M]; N],
        }
    }

    pub fn new_matrix(data: [[f64; M]; N]) -> Self {
        Self {
            data: data,
        }
    }
}
