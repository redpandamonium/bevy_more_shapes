use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use crate::util::FlatTrapezeIndices;

pub struct Grid {
    /// Length along the x axis
    pub width: f32,
    /// Length along the z axis
    pub height: f32,
    /// Segments on the x axis
    pub width_segments: usize,
    /// Segments on the z axis
    pub height_segments: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            width: 1.0,
            height: 1.0,
            width_segments: 1,
            height_segments: 1
        }
    }
}

impl Grid {
    pub fn new_square(length: f32, segments: usize) -> Self {
        Self {
            width: length,
            height: length,
            width_segments: segments,
            height_segments: segments
        }
    }
}

impl From<Grid> for Mesh {
    fn from(grid: Grid) -> Self {

        // Validate input parameters
        assert!(grid.width_segments > 0, "A grid must have segments");
        assert!(grid.height_segments > 0, "A grid must have segments");
        assert!(grid.width > 0.0, "A grid must have positive width");
        assert!(grid.height > 0.0, "A grid must have positive height");

        let num_points = (grid.height_segments + 1) * (grid.width_segments + 1);
        let num_faces = grid.height_segments * grid.width_segments;

        let mut indices : Vec<u32> = Vec::with_capacity(6 * num_faces); // two triangles per rectangle
        let mut positions : Vec<[f32; 3]> = Vec::with_capacity(num_points);
        let mut uvs : Vec<[f32; 2]> = Vec::with_capacity(num_points);
        let mut normals : Vec<[f32; 3]> = Vec::with_capacity(num_points);

        // This is used to center the grid on the origin
        let width_half = grid.width / 2.0;
        let height_half = grid.height / 2.0;

        // The length of a single segment
        let x_segment_len = grid.width / grid.width_segments as f32;
        let z_segment_len = grid.height / grid.height_segments as f32;

        // The inverse of the segment lengths
        let width_segments_inv = 1.0 / grid.width_segments as f32;
        let height_segments_inv = 1.0 / grid.height_segments as f32;

        // Generate vertices
        for z in 0..grid.height_segments + 1 {
            for x in 0..grid.width_segments + 1 {

                positions.push([x as f32 * x_segment_len - width_half, 0.0, z as f32 * z_segment_len - height_half]);
                uvs.push([x as f32 * width_segments_inv, z as f32 * height_segments_inv]);
                normals.push([0.0, 1.0, 0.0]);
            }
        }

        // Generate indices
        for face_z in 0..grid.height_segments {
            for face_x in 0..grid.width_segments {

                let lower_left = face_z * (grid.width_segments + 1) + face_x;
                let face = FlatTrapezeIndices {
                    lower_left: lower_left as u32,
                    upper_left: (lower_left + (grid.width_segments + 1)) as u32,
                    lower_right: (lower_left + 1) as u32,
                    upper_right: (lower_left + 1 + (grid.width_segments + 1)) as u32,
                };
                face.generate_triangles(&mut indices);
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}