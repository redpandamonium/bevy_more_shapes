// This is based on a blog post found here: http://apparat-engine.blogspot.com/2013/04/procdural-meshes-cylinder.html.

use bevy::math::Vec3;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use std::slice::Iter;
use crate::util::FlatTrapezeIndices;

pub struct Cylinder {
    pub height: f32,
    pub radius_bottom: f32,
    pub radius_top: f32,
    pub subdivisions: u32,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            height: 1.0,
            radius_bottom: 0.5,
            radius_top: 0.5,
            subdivisions: 32,
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
            subdivisions,
        }
    }
}

enum VertexPass {
    Top,
    Bottom,
    TopRing,
    BottomRing,
}

impl VertexPass {
    pub fn iter() -> Iter<'static, VertexPass> {
        static VALUES: [VertexPass; 4] = [
            VertexPass::Top,
            VertexPass::Bottom,
            VertexPass::TopRing,
            VertexPass::BottomRing,
        ]; // order is important, top vertices come first
        VALUES.iter()
    }
}

fn add_indices_top(indices: &mut Vec<u32>, mid_idx: u32, cylinder: &Cylinder) {
    for i in 0..cylinder.subdivisions - 1 {
        let lt = i;
        let rt = i + 1;

        indices.push(mid_idx);
        indices.push(rt);
        indices.push(lt);
    }

    // Fix gap where the last vertex meets the first
    indices.push(mid_idx);
    indices.push(0);
    indices.push(cylinder.subdivisions - 1);
}

fn add_indices_bottom(indices: &mut Vec<u32>, mid_idx: u32, cylinder: &Cylinder) {
    let base_index_bottom = cylinder.subdivisions;

    for i in 0..cylinder.subdivisions - 1 {
        let lb = i + base_index_bottom;
        let rb = i + 1 + base_index_bottom;

        indices.push(lb);
        indices.push(rb);
        indices.push(mid_idx);
    }

    // Fix gap where the last vertex meets the first
    indices.push(base_index_bottom + cylinder.subdivisions - 1);
    indices.push(base_index_bottom);
    indices.push(mid_idx);
}

fn add_indices_body(indices: &mut Vec<u32>, cylinder: &Cylinder) {
    let base_index_top_ring = cylinder.subdivisions * 2;
    let base_index_bottom_ring = cylinder.subdivisions * 3;

    for i in 0..cylinder.subdivisions - 1 {

        let face = FlatTrapezeIndices {
            lower_left: i + base_index_bottom_ring,
            upper_left: i + base_index_top_ring,
            lower_right: i + 1 + base_index_bottom_ring,
            upper_right: i + 1 + base_index_top_ring,
        };
        face.generate_triangles(indices);
    }

    // Fix gap where the last vertex meets the first
    let face = FlatTrapezeIndices {
        lower_left: base_index_bottom_ring + cylinder.subdivisions - 1,
        upper_left: base_index_top_ring + cylinder.subdivisions - 1,
        lower_right: base_index_bottom_ring,
        upper_right: base_index_top_ring,
    };
    face.generate_triangles(indices);
}

// https://en.wikipedia.org/wiki/UV_mapping
fn sphere_coordinates(sphere_coord: Vec3) -> [f32; 2] {
    let u = 0.5 + (f32::atan2(sphere_coord.x, sphere_coord.z) / (2.0 * std::f32::consts::PI));
    let v = 0.5 + f32::asin(sphere_coord.y) / std::f32::consts::PI;
    [u, v]
}

impl From<Cylinder> for Mesh {
    fn from(cylinder: Cylinder) -> Self {

        // Input parameter validation
        assert!(cylinder.height > 0.0, "Must have positive height");
        assert!(cylinder.radius_bottom >= 0.0, "Must have positive radius.");
        assert!(cylinder.radius_top >= 0.0, "Must have positive radius.");
        assert!(cylinder.subdivisions > 2, "Must have at least 3 subdivisions to close the surface.");

        // Vertex order in the buffer:
        // 1: n_subdivisions top face
        // 2: n_subdivisions bottom face
        // 3: n_subdivisions top outer ring
        // 4: n_subdivisions bottom outer ring
        // 5: top mid vertex
        // 6: bottom mid vertex

        let num_vertices = cylinder.subdivisions * 4 + 2;
        let num_indices = cylinder.subdivisions * 2 * 6;
        let angle_step = 2.0 * std::f32::consts::PI / cylinder.subdivisions as f32;

        let mut vertices = Vec::with_capacity(num_vertices as usize);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices as usize);
        let mut normals = Vec::with_capacity(num_vertices as usize);
        let mut indices = Vec::with_capacity(num_indices as usize);

        // Create all vertices

        // Ring vertices
        for pass in VertexPass::iter() {
            for row_idx in 0..cylinder.subdivisions {
                let theta = angle_step * row_idx as f32;

                let height = match pass {
                    VertexPass::Top => cylinder.height / 2.0,
                    VertexPass::Bottom => -cylinder.height / 2.0,
                    VertexPass::TopRing => cylinder.height / 2.0,
                    VertexPass::BottomRing => -cylinder.height / 2.0,
                };

                let radius = match pass {
                    VertexPass::Top => cylinder.radius_top,
                    VertexPass::Bottom => cylinder.radius_bottom,
                    VertexPass::TopRing => cylinder.radius_top,
                    VertexPass::BottomRing => cylinder.radius_bottom,
                };

                let position =
                    Vec3::new(radius * f32::cos(theta), height, radius * f32::sin(theta));

                let normal = match pass {
                    VertexPass::Top => Vec3::new(0.0, 1.0, 0.0),
                    VertexPass::Bottom => Vec3::new(0.0, -1.0, 0.0),
                    VertexPass::TopRing => Vec3::new(position.x, 0.0, position.z).normalize(),
                    VertexPass::BottomRing => Vec3::new(position.x, 0.0, position.z).normalize(),
                };

                vertices.push(position.to_array());
                normals.push(normal.to_array());

                let mut uv = sphere_coordinates(-position.normalize());
                uv[0] = if uv[0] < 0.0 {
                    1.0 - uv[0]
                } else if uv[0] > 1.0 {
                    uv[0] - 1.0
                } else {
                    uv[0]
                };
                uvs.push(uv);
                // TODO: UVs
            }
        }

        // Ring center vertices
        vertices.push([0.0, cylinder.height / 2.0, 0.0]); // top
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5, 0.0]); // We compute them manually because the the algorithm we use is undefined at x=0,y=0.
        vertices.push([0.0, -cylinder.height / 2.0, 0.0]); // bottom
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5, 1.0]);

        // Add the indices
        let top_mid_idx = (vertices.len() - 2) as u32;
        let bottom_mid_idx = (vertices.len() - 1) as u32;
        add_indices_top(&mut indices, top_mid_idx, &cylinder);
        add_indices_bottom(&mut indices, bottom_mid_idx, &cylinder);
        add_indices_body(&mut indices, &cylinder);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
