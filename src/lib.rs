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

pub use crate::cone::Cone;
pub use crate::cylinder::Cylinder;
pub use crate::grid::Grid;
pub use crate::polygon::Polygon;
pub use crate::torus::Torus;