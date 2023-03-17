// This is based on a blog post found here: http://apparat-engine.blogspot.com/2013/04/procdural-meshes-cylinder.html.

use bevy::math::Vec3;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use crate::MeshData;
use crate::util::FlatTrapezeIndices;

pub struct Cylinder {
    pub height: f32,
    pub radius_bottom: f32,
    pub radius_top: f32,
    pub radial_segments: u32,
    pub height_segments: u32,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            height: 1.0,
            radius_bottom: 0.5,
            radius_top: 0.5,
            radial_segments: 32,
            height_segments: 1,
        }
    }
}

impl Cylinder {
    /// Create a cylinder where the top and bottom disc have the same radius.
    pub fn new_regular(height: f32, radius: f32, subdivisions: u32) -> Self {
        Self {
            height,
            radius_bottom: radius,
            radius_top: radius,
            radial_segments: subdivisions,
            height_segments: 1,
        }
    }
}

fn add_top(mesh: &mut MeshData, cylinder: &Cylinder) {

    let angle_step = std::f32::consts::TAU / cylinder.radial_segments as f32;
    let base_index = mesh.positions.len() as u32;

    // Center
    let center_pos = Vec3::new(0.0, cylinder.height / 2.0, 0.0);
    mesh.positions.push(center_pos.to_array());
    mesh.uvs.push([0.5, 0.5]);
    mesh.normals.push(Vec3::Y.to_array());

    // Vertices
    for i in 0..=cylinder.radial_segments {

        let theta = i as f32 * angle_step;
        let x_unit = f32::cos(theta);
        let z_unit = f32::sin(theta);

        let pos = Vec3::new(
            cylinder.radius_top * x_unit,
            cylinder.height / 2.0,
            cylinder.radius_top * z_unit,
        );
        let uv = [
            (z_unit * 0.5) + 0.5,
            (x_unit * 0.5) + 0.5,
        ];

        mesh.positions.push(pos.to_array());
        mesh.uvs.push(uv);
        mesh.normals.push(Vec3::Y.to_array())
    }

    // Indices
    for i in 0..cylinder.radial_segments {
        mesh.indices.push(base_index);
        mesh.indices.push(base_index + i + 2);
        mesh.indices.push(base_index + i + 1);
    }
}

fn add_bottom(mesh: &mut MeshData, cylinder: &Cylinder) {

    let angle_step = std::f32::consts::TAU / cylinder.radial_segments as f32;
    let base_index = mesh.positions.len() as u32;

    // Center
    let center_pos = Vec3::new(0.0, -cylinder.height / 2.0, 0.0);
    mesh.positions.push(center_pos.to_array());
    mesh.uvs.push(uvs(center_pos));
    mesh.normals.push((-Vec3::Y).to_array());

    // Vertices
    for i in 0..=cylinder.radial_segments {

        let theta = i as f32 * angle_step;
        let x_unit = f32::cos(theta);
        let z_unit = f32::sin(theta);

        let pos = Vec3::new(
            cylinder.radius_bottom * x_unit,
            -cylinder.height / 2.0,
            cylinder.radius_bottom * z_unit,
        );
        let uv = [
            (z_unit * 0.5) + 0.5,
            (x_unit * -0.5) + 0.5,
        ];

        mesh.positions.push(pos.to_array());
        mesh.uvs.push(uv);
        mesh.normals.push((-Vec3::Y).to_array())
    }

    // Indices
    for i in 0..cylinder.radial_segments {
        mesh.indices.push(base_index + i + 1);
        mesh.indices.push(base_index + i + 2);
        mesh.indices.push(base_index);
    }
}

fn add_body(mesh: &mut MeshData, cylinder: &Cylinder) {

    let angle_step = std::f32::consts::TAU / cylinder.radial_segments as f32;
    let base_index = mesh.positions.len() as u32;

    // Vertices
    for i in 0..=cylinder.radial_segments {

        let theta = angle_step * i as f32;
        let x_unit = f32::cos(theta);
        let z_unit = f32::sin(theta);

        // Calculate normal of this segment, it's a straight line so all normals are the same
        let slope = (cylinder.radius_bottom - cylinder.radius_top) / cylinder.height;
        let normal = Vec3::new(x_unit, slope, z_unit).normalize();

        for h in 0..=cylinder.height_segments {
            let height_percent = h as f32 / cylinder.height_segments as f32;
            let y = height_percent * cylinder.height - cylinder.height / 2.0;
            let radius = (1.0 - height_percent) * cylinder.radius_bottom + height_percent * cylinder.radius_top;

            let pos = Vec3::new(x_unit * radius, y, z_unit * radius);
            let uv = [i as f32 / cylinder.radial_segments as f32, height_percent];

            mesh.positions.push(pos.to_array());
            mesh.normals.push(normal.to_array());
            mesh.uvs.push(uv);
        }
    }

    // Indices
    for i in 0..cylinder.radial_segments {
        for h in 0..cylinder.height_segments {
            let segment_base = base_index + (i * (cylinder.height_segments + 1)) + h;
            let indices = FlatTrapezeIndices {
                lower_left: segment_base,
                upper_left: segment_base + 1,
                lower_right: segment_base + cylinder.height_segments + 1,
                upper_right: segment_base + cylinder.height_segments + 2,
            };
            indices.generate_triangles(&mut mesh.indices);
        }
    }
}

// https://en.wikipedia.org/wiki/UV_mapping
fn sphere_coordinates(sphere_coord: Vec3) -> [f32; 2] {
    let u = 0.5 + (f32::atan2(sphere_coord.x, sphere_coord.z) / (2.0 * std::f32::consts::PI));
    let v = 0.5 + f32::asin(sphere_coord.y) / std::f32::consts::PI;
    [u, v]
}

fn uvs(pos: Vec3) -> [f32; 2] {
    let mut uv = sphere_coordinates(pos);
    uv[0] = if uv[0] < 0.0 {
        1.0 - uv[0]
    } else if uv[0] > 1.0 {
        uv[0] - 1.0
    } else {
        uv[0]
    };
    uv
}

impl From<Cylinder> for Mesh {
    fn from(cylinder: Cylinder) -> Self {

        // Input parameter validation
        assert_ne!(cylinder.radius_top, 0.0, "Radius must not be 0. Use a cone instead.");
        assert_ne!(cylinder.radius_bottom, 0.0, "Radius must not be 0. Use a cone instead.");
        assert!(cylinder.radius_bottom > 0.0, "Must have positive radius.");
        assert!(cylinder.radius_top > 0.0, "Must have positive radius.");
        assert!(cylinder.radial_segments > 2, "Must have at least 3 subdivisions to close the surface.");
        assert!(cylinder.height_segments >= 1, "Must have at least one height segment.");
        assert!(cylinder.height > 0.0, "Must have positive height");

        // Vertex order in the buffer:
        // 1: n_subdivisions top face
        // 2: n_subdivisions bottom face
        // 3: n_subdivisions top outer ring
        // 4: n_subdivisions bottom outer ring
        // 5: top mid vertex
        // 6: bottom mid vertex

        let num_vertices = cylinder.radial_segments * 4 + 2;
        let num_indices = cylinder.radial_segments * 2 * 6;

        let mut mesh = MeshData {
            positions: Vec::with_capacity(num_vertices as usize),
            normals: Vec::with_capacity(num_vertices as usize),
            uvs: Vec::with_capacity(num_vertices as usize),
            indices: Vec::with_capacity(num_indices as usize),
        };

        add_top(&mut mesh, &cylinder);
        add_bottom(&mut mesh, &cylinder);
        add_body(&mut mesh, &cylinder);

        let mut m = Mesh::new(PrimitiveTopology::TriangleList);
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uvs);
        m.set_indices(Some(Indices::U32(mesh.indices)));
        m
    }
}
