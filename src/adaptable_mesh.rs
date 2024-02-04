use core::fmt;
use std::cmp::max;

struct AdaptableMesh<const X: usize, const Y: usize, const Z: usize, T: Default + Copy + fmt::Debug, F: Fn(T) -> T> {
    mesh: Box<[[[MeshNode<T>; X]; Y]; Z]>,
    dimensions: Vec3,
    f: F,
}

impl<const X: usize, const Y: usize, const Z: usize, T: Default + Copy + fmt::Debug, F: Fn(T) -> T> AdaptableMesh<X, Y, Z, T, F> {
    fn new(update_function: F, dimensions: Vec3) -> Self {
        let mut mesh = Box::new([[[MeshNode::<T>::default(); X]; Y]; Z]);

        let x_step = dimensions.x / max(X as i64 - 1, 1) as f64;
        let y_step = dimensions.y / max(Y as i64 - 1, 1) as f64;
        let z_step = dimensions.z / max(Z as i64 - 1, 1) as f64;

        (*mesh).iter_mut().enumerate().for_each(|(z, xy_mesh)| {
            xy_mesh.iter_mut().enumerate().for_each(|(y, x_mesh)| {
                x_mesh.iter_mut().enumerate().for_each(|(x, mesh_node)| {
                    mesh_node.point = Vec3 {
                        x: (x as f64) * x_step,
                        y: (y as f64) * y_step,
                        z: (z as f64) * z_step,
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

    fn update(&mut self) {
        (*self.mesh).iter_mut().enumerate().for_each(|(z, xy_mesh)| {
            xy_mesh.iter_mut().enumerate().for_each(|(y, x_mesh)| {
                x_mesh.iter_mut().enumerate().for_each(|(x, mesh_node)| {
                    mesh_node.value = (self.f)(mesh_node.value);
                });
            });
        });
    }
}

impl<const X: usize, const Y: usize, const Z: usize, T: Default + Copy + fmt::Debug, F: Fn(T) -> T> fmt::Debug for AdaptableMesh<X, Y, Z, T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdaptableMesh")
         .field("mesh", &self.mesh)
         .finish()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct MeshNode<T: Default + Copy> {
    point: Vec3,
    value: T,
}

#[derive(Debug, Default, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub fn main() {
    let mut simulation = AdaptableMesh::<3, 3, 1, f64, _>::new(
        |f| f.powi(2) + 1.0,
        Vec3 {x: 10.0, y: 10.0, z: 10.0},
    );
    simulation.update();
    println!("{:?}", simulation);
}
