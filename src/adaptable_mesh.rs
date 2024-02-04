struct AdaptableMesh<const X: usize, const Y: usize, const Z: usize, T: Default + Copy, F: Fn(T) -> T> {
    mesh: Box<[[[MeshNode<T>; X]; Y]; Z]>,
    dimensions: Vec3,
    f: F,
}

impl<const X: usize, const Y: usize, const Z: usize, T: Default + Copy, F: Fn(T) -> T> AdaptableMesh<X, Y, Z, T, F> {
    fn new(update_function: F, dimensions: Vec3) -> Self {
        let mut mesh = Box::new([[[MeshNode::<T>::default(); X]; Y]; Z]);

        (*mesh).iter_mut().enumerate().for_each(|(z, xy_mesh)| {
            xy_mesh.iter_mut().enumerate().for_each(|(y, x_mesh)| {
                x_mesh.iter_mut().enumerate().for_each(|(x, mesh_node)| {
                    mesh_node.point = Vec3 {
                        x: (x as f64) * dimensions.x / (X as f64),
                        y: (y as f64) * dimensions.y / (Y as f64),
                        z: (z as f64) * dimensions.z / (Z as f64),
                    };
                });
            });
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

#[derive(Default, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub fn main() {
    let simulation = AdaptableMesh::<100, 100, 10, f64, _>::new(
        |f| f.powi(2),
        Vec3 {x: 10.0, y: 10.0, z: 10.0},
    );
}
