use bevy::math::{Rect, Vec2, Vec3};
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use triangulate::{FanBuilder, PolygonList, TriangleWinding, Triangulate, TriangulateDefault, TriangulationError, Vertex};
use triangulate::builders::{FanToListAdapter, VecDelimitedIndexedFanBuilder, VecIndexedListBuilder, VecVecFanBuilder};

pub struct Polygon {
    /// Points on a path where the last and first point are connected to form a closed circle.
    /// Must not intersect. Must contain enough points.
    pub points: Vec<Vec2>,
}

impl Polygon {

    pub fn new_regular_ngon(radius: f32, n: usize) -> Polygon {
        let angle_step = 2.0 * std::f32::consts::PI / n as f32;
        let mut points = Vec::with_capacity(n);

        for i in 0..n {
            let theta = angle_step * i as f32;
            points.push(Vec2::new(radius * f32::cos(theta), radius * f32::sin(theta)));
        }

        Polygon {
            points
        }
    }

    pub fn new_triangle(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 3)
    }

    pub fn new_pentagon(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 5)
    }

    pub fn new_hexagon(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 6)
    }

    pub fn new_octagon(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 8)
    }
}

fn bounding_rect_for_points<'a>(points: impl Iterator<Item=&'a Vec2>) -> Rect<f32> {

    let mut x_min = 0.0f32;
    let mut x_max = 0.0f32;
    let mut y_min = 0.0f32;
    let mut y_max = 0.0f32;

    for point in points {
        x_min = x_min.min(point.x);
        x_max = x_max.max(point.x);
        y_min = y_min.min(point.y);
        y_max = y_max.max(point.y);
    }

    Rect {
        left: x_min,
        right: x_max,
        top: y_max,
        bottom: y_min
    }
}

// This is an ugly workaround for rust's orphan rule. Neither Vec2 nor the Vertex trait come from this crate.
// So we need to implement a newtype and hope it gets optimized away (which it should).
#[derive(Debug, Copy, Clone)]
struct Vec2f(Vec2);

impl Vertex for Vec2f {
    type Coordinate = f32;

    fn x(&self) -> Self::Coordinate {
        self.0.x
    }

    fn y(&self) -> Self::Coordinate {
        self.0.y
    }
}

impl From<Polygon> for Mesh {
    fn from(polygon: Polygon) -> Self {

        let mut positions : Vec<[f32; 3]> = Vec::with_capacity(polygon.points.len());
        let mut normals : Vec<[f32; 3]> = Vec::with_capacity(polygon.points.len());
        let mut uvs : Vec<[f32; 2]> = Vec::with_capacity(polygon.points.len());

        // The domain is needed for UV mapping. The domain tells us how to transform all points to optimally fit the 0-1 range.
        let domain = bounding_rect_for_points(polygon.points.iter());

        // Add the vertices
        for v in &polygon.points {
            positions.push([v.x, 0.0, v.y]);
            normals.push(Vec3::Y.to_array());

            // Transform the polygon domain to the 0-1 UV domain.
            let u = (v.x - domain.left) / (domain.right - domain.left);
            let v = (v.y - domain.bottom) / (domain.top - domain.bottom);
            uvs.push([u, v]);
        }

        // Triangulate to obtain the indices
        // This library is terrible to use. The heck is that initializer object.
        let polygons = polygon.points.into_iter().map(|v| Vec2f(v)).collect::<Vec<Vec2f>>();
        let mut null_obj = vec![];
        let result = polygons.triangulate::<FanToListAdapter<_, VecIndexedListBuilder<_>>>(&mut null_obj).unwrap();
        let indices = result.iter().map(|i| *i as u32).collect();

        // Put the mesh together
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}