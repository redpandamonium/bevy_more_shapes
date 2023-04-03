use bevy::math::Vec3;
use bevy::prelude::{Mesh, Vec2};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::MeshData;
use crate::util::FlatTrapezeIndices;

pub struct Torus {
    /// The radius of the ring. Measured from the mesh's origin to the center line of the tube.
    pub radius: f32,
    /// The width of the ring
    pub tube_radius: f32,
    /// The number of segments that make up the ring.
    pub radial_segments: usize,
    /// The number of segments that make up the tube.
    pub tube_segments: usize,
    /// Circumference in radians around the main axis. 2pi for a full torus.
    pub radial_circumference: f32,
    /// Circumference in radians of the individual ring segments. 2pi for a closed tube.
    pub tube_circumference: f32,
    /// The offset in radians of where on the circle the torus begins. Ignored if radial_circumference is 2pi.
    pub radial_offset: f32,
    /// The offset in radians of where the tube begins on its circle. Ignored if tube_circumference is 2pi.
    pub tube_offset: f32,
}

impl Default for Torus {
    fn default() -> Self {
        Self {
            radius: 0.8,
            tube_radius: 0.2,
            radial_segments: 64,
            tube_segments: 32,
            radial_circumference: std::f32::consts::TAU,
            tube_circumference: std::f32::consts::TAU,
            radial_offset: 0.0,
            tube_offset: 0.0,
        }
    }
}

impl From<Torus> for Mesh {
    fn from(torus: Torus) -> Mesh {

        // Input parameter validation
        assert!(torus.radius > 0.0, "The radii of a torus must be positive");
        assert!(torus.tube_radius > 0.0, "The radii of a torus must be positive");
        assert!(torus.radial_segments >= 3, "Must have at least 3 radial segments");
        assert!(torus.tube_segments >= 3, "3 Must have at least 3 tube segments");
        assert!(torus.radial_circumference > 0.0, "Radial circumference must be positive");
        assert!(torus.tube_circumference > 0.0, "Tube circumference must be positive");
        assert!(torus.radial_circumference <= std::f32::consts::TAU, "Radial circumference must not exceed 2pi radians");
        assert!(torus.tube_circumference <= std::f32::consts::TAU, "Tube circumference must not exceed 2pi radians");
        if torus.radial_circumference < std::f32::consts::TAU {
            assert!(torus.radial_offset >= 0.0, "Radial offset must be between 0 and 2pi");
            assert!(torus.radial_offset <= std::f32::consts::TAU, "Radial offset must be between 0 and 2pi");
        }
        if torus.tube_radius < std::f32::consts::TAU {
            assert!(torus.tube_radius >= 0.0, "Tube offset must be between 0 and 2pi");
            assert!(torus.tube_radius <= std::f32::consts::TAU, "Tube offset must be between 0 and 2pi");
        }

        let num_vertices = (torus.radial_segments + 1) * (torus.tube_segments + 1);
        let mut mesh = MeshData {
            positions: Vec::with_capacity(num_vertices),
            normals:  Vec::with_capacity(num_vertices),
            uvs: Vec::with_capacity(num_vertices),
            indices: Vec::with_capacity(torus.radial_segments * torus.tube_segments * 6),
        };
        
        generate_torus_body(&mut mesh, &torus);

        let mut m = Mesh::new(PrimitiveTopology::TriangleList);
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uvs);
        m.set_indices(Some(Indices::U32(mesh.indices)));
        m
    }
}

fn generate_torus_body(mesh: &mut MeshData, torus: &Torus) {

    // This code is based on http://apparat-engine.blogspot.com/2013/04/procedural-meshes-torus.html
    
    let angle_step_vertical = torus.tube_circumference / torus.tube_segments as f32;
    let angle_step_horizontal = torus.radial_circumference / torus.radial_segments as f32;

    // Add vertices ring by ring
    for horizontal_idx in 0..=torus.radial_segments {

        let theta_horizontal = angle_step_horizontal * horizontal_idx as f32 + torus.radial_offset;

        // The center of the vertical ring
        let ring_center = Vec3::new(
            torus.radius * f32::cos(theta_horizontal),
            0.0,
            torus.radius * f32::sin(theta_horizontal)
        );

        for vertical_idx in 0..=torus.tube_segments {

            let theta_vertical = angle_step_vertical * vertical_idx as f32 + torus.tube_offset;

            let position = Vec3::new(
                f32::cos(theta_horizontal) * (torus.radius + torus.tube_radius * f32::cos(theta_vertical)),
                f32::sin(theta_vertical) * torus.tube_radius,
                f32::sin(theta_horizontal) * (torus.radius + torus.tube_radius * f32::cos(theta_vertical)),
            );

            // The normal points from the radius 0 torus to the actual point
            let normal = (position - ring_center).normalize();
            mesh.positions.push(position);
            mesh.normals.push(normal);

            // Since the segments are basically a deformed grid, we can overlay that onto the UV space
            let u = 1.0 / torus.radial_segments as f32 * horizontal_idx as f32;
            let v = 1.0 / torus.tube_segments as f32 * vertical_idx as f32;
            mesh.uvs.push(Vec2::new(u, v));
        }
    }

    // Add indices for each face
    for horizontal_idx in 0..torus.radial_segments {

        let ring0_base_idx = horizontal_idx * (torus.tube_segments + 1);
        let ring1_base_idx = (horizontal_idx + 1) * (torus.tube_segments + 1);

        for vertical_idx in 0..torus.tube_segments {
            let face = FlatTrapezeIndices {
                lower_left: (ring0_base_idx + vertical_idx) as u32,
                upper_left: (ring0_base_idx + vertical_idx + 1) as u32,
                lower_right: (ring1_base_idx + vertical_idx) as u32,
                upper_right: (ring1_base_idx + vertical_idx + 1) as u32,
            };
            face.generate_triangles(&mut mesh.indices);
        }
    }
}