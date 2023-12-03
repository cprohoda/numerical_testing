pub fn main() {
    let a = Matrix::<2, 2>::new_matrix([[1., 0.], [0., 1.]]);
    let b = Matrix::<2, 2>::new_matrix([[1., 1.], [2., 3.]]);
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("Result: {:?}", a.multiply(b));
}

#[derive(Debug)]
pub struct Matrix<const M: usize, const N: usize> {
    pub data: [[f64; M]; N],
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

    pub fn multiply(&self, other: Matrix<N, M>) -> Matrix<M, M> {
        let mut result = Matrix::<M, M>::new(0.);

        result.data.iter_mut().enumerate().for_each(|(i, mut row)| {
            row.iter_mut().enumerate().for_each(|(j, mut value)| {
                *value = self.data[i].iter().enumerate().map(|(k, val)| {
                    val * other.data[j][k]
                }).sum();
            });
        });

        result
    }
}
