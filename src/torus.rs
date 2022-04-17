use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub struct Torus {
    /// The radius to the ring from the center
    radius: f32,
    /// The radius of the torus ring
    ring_radius: f32,
    /// The number of segments around horizontal big ring
    horizontal_segments: usize,
    /// The number of segments around the vertical small rings
    vertical_segments: usize,
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

        let num_vertices = torus.horizontal_segments * torus.vertical_segments;
        let mut positions : Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals : Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs : Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices = Vec::with_capacity(num_vertices * 6); // num_vertices faces, each face is a flat trapeze with 2 triangles

        let angle_step_vertical = 2.0 * std::f32::consts::PI / torus.vertical_segments as f32;
        let angle_step_horizontal = 2.0 * std::f32::consts::PI / torus.horizontal_segments as f32;

        // Add vertices ring by ring
        for horizontal_idx in 0..torus.horizontal_segments {

            let theta_horizontal = angle_step_horizontal * horizontal_idx as f32;

            // The center of the vertical ring
            let ring_center = Vec3::new(
                torus.radius * f32::cos(theta_horizontal),
                0.0,
                torus.radius * f32::sin(theta_horizontal)
            );

            for vertical_idx in 0..torus.vertical_segments {

                let theta_vertical = angle_step_vertical * vertical_idx as f32;
                let position = Vec3::new(
                    f32::cos(theta_vertical) * (torus.radius + torus.ring_radius * f32::cos(theta_horizontal)),
                    f32::sin(theta_vertical) * (torus.radius + torus.ring_radius * f32::cos(theta_horizontal)),
                    f32::sin(theta_horizontal) * torus.ring_radius
                );
                let normal = (position - ring_center).normalize();
                positions.push(position.to_array());
                normals.push(normal.to_array());
                // TODO: uvs
                uvs.push([0.0, 0.0]);
            }
        }

        // Add indices for each face

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}