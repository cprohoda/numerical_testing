struct AdaptableMesh<const X: usize, const Y: usize, const Z: usize, T: Default + Copy, F: Fn(T) -> T> {
    mesh: [[[MeshNode<T>; X]; Y]; Z],
    dimensions: Vec3,
    f: F,
}

impl<const X: usize, const Y: usize, const Z: usize, T: Default + Copy, F: Fn(T) -> T> AdaptableMesh<X, Y, Z, T, F> {
    fn new(update_function: F, dimensions: Vec3) -> Self {
        let mut mesh = [[[MeshNode::<T>::default(); X]; Y]; Z];

        (0..X).for_each(|x| {
            (0..Y).for_each(|y| {
                (0..Z).for_each(|z| {
                    mesh[x][y][z].point = Vec3 {
                        x: (x as f64) * dimensions.x / (X as f64),
                        y: (y as f64) * dimensions.y / (Y as f64),
                        z: (z as f64) * dimensions.z / (Z as f64),
                    };
                })
            })
        });

        Self {
            mesh: mesh,
            dimensions: dimensions,
            f: update_function,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct MeshNode<T: Default + Copy> {
    point: Vec3,
    value: T,
}

impl<T: Default + Copy> MeshNode<T> {
    fn new(point: Vec3) -> Self {
        Self {
            point: point,
            value: T::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub fn main() {
}
