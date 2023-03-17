pub mod cone;
pub mod cylinder;
pub mod grid;
pub mod polygon;
pub mod torus;
pub(crate) mod util;

struct MeshData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl MeshData {
    fn new(num_vertices: usize, num_indices: usize) -> Self {
        Self {
            positions: Vec::with_capacity(num_vertices as usize),
            normals: Vec::with_capacity(num_vertices as usize),
            uvs: Vec::with_capacity(num_vertices as usize),
            indices: Vec::with_capacity(num_indices as usize),
        }
    }
}

pub use crate::cone::Cone;
pub use crate::cylinder::Cylinder;
pub use crate::grid::Grid;
pub use crate::polygon::Polygon;
pub use crate::torus::Torus;