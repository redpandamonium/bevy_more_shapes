use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::util::FlatTrapezeIndices;

pub struct Torus {
    /// The radius to the ring from the center
    pub radius: f32,
    /// The radius of the torus ring
    pub ring_radius: f32,
    /// The number of segments around horizontal big ring
    pub horizontal_segments: usize,
    /// The number of segments around the vertical small rings
    pub vertical_segments: usize,
}

impl Default for Torus {
    fn default() -> Self {
        Self {
            radius: 0.8,
            ring_radius: 0.2,
            horizontal_segments: 32,
            vertical_segments: 16
        }
    }
}

impl From<Torus> for Mesh {
    fn from(torus: Torus) -> Mesh {

        // Input parameter validation
        assert!(torus.radius > 0.0, "The radii of a torus must be positive");
        assert!(torus.ring_radius > 0.0, "The radii of a torus must be positive");
        assert!(torus.horizontal_segments >= 3, "3 segments are needed to produce a closed shape.");
        assert!(torus.vertical_segments >= 3, "3 segments are needed to produce a closed shape.");

        let num_vertices = (torus.horizontal_segments + 1) * (torus.vertical_segments + 1);
        let mut positions : Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals : Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs : Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices = Vec::with_capacity(torus.horizontal_segments * torus.vertical_segments * 6);

        let angle_step_vertical = 2.0 * std::f32::consts::PI / torus.vertical_segments as f32;
        let angle_step_horizontal = 2.0 * std::f32::consts::PI / torus.horizontal_segments as f32;

        // Add vertices ring by ring
        for horizontal_idx in 0..=torus.horizontal_segments {

            let theta_horizontal = angle_step_horizontal * horizontal_idx as f32;

            // The center of the vertical ring
            let ring_center = Vec3::new(
                torus.radius * f32::cos(theta_horizontal),
                0.0,
                torus.radius * f32::sin(theta_horizontal)
            );

            for vertical_idx in 0..=torus.vertical_segments {

                let theta_vertical = angle_step_vertical * vertical_idx as f32;
                let position = Vec3::new(
                    f32::cos(theta_horizontal) * (torus.radius + torus.ring_radius * f32::cos(theta_vertical)),
                    f32::sin(theta_vertical) * torus.ring_radius,
                    f32::sin(theta_horizontal) * (torus.radius + torus.ring_radius * f32::cos(theta_vertical)),
                );
                // The normal points from the radius 0 torus to the actual point
                let normal = (position - ring_center).normalize();
                positions.push(position.to_array());
                normals.push(normal.to_array());

                // TODO: uvs
                uvs.push([0.0, 0.0]);
            }
        }

        // Add indices for each face
        for horizontal_idx in 0..torus.horizontal_segments {

            let ring0_base_idx = horizontal_idx * (torus.vertical_segments + 1);
            let ring1_base_idx = (horizontal_idx + 1) * (torus.vertical_segments + 1);

            for vertical_idx in 0..torus.vertical_segments {
                let face = FlatTrapezeIndices {
                    lower_left: (ring0_base_idx + vertical_idx) as u32,
                    upper_left: (ring0_base_idx + vertical_idx + 1) as u32,
                    lower_right: (ring1_base_idx + vertical_idx) as u32,
                    upper_right: (ring1_base_idx + vertical_idx + 1) as u32,
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