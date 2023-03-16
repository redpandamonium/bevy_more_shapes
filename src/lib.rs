pub mod cone;
pub mod cylinder;
pub mod grid;
pub mod polygon;
pub mod torus;
pub(crate) mod util;

/// Settings that apply to all shapes.
pub struct GenerationSettings {
    pub uvs: bool,
    pub normals: bool,
    /// Duplicate some vertices when it allows easier texturing
    pub duplicate_vertices: bool,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            uvs: true,
            normals: true,
            duplicate_vertices: true,
        }
    }
}

pub use crate::cone::Cone;
pub use crate::cylinder::Cylinder;
pub use crate::grid::Grid;
pub use crate::polygon::Polygon;