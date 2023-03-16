use std::error::Error;
use std::fmt::{Display, Formatter};
use bevy::math::{Rect, Vec2, Vec3};
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use triangulate::{ListFormat, TriangulationError, Vertex};
use triangulate::formats::IndexedListFormat;

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
            points.push(Vec2::new(
                radius * f32::cos(theta),
                radius * f32::sin(theta),
            ));
        }

        Polygon { points }
    }

    /// Creates a triangle where the points touch a circle of specified radius.
    pub fn new_triangle(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 3)
    }

    /// Creates a pentagon where the points touch a circle of specified radius.
    pub fn new_pentagon(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 5)
    }

    /// Creates a hexagon where the points touch a circle of specified radius.
    pub fn new_hexagon(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 6)
    }

    /// Creates a octagon where the points touch a circle of specified radius.
    pub fn new_octagon(radius: f32) -> Polygon {
        Self::new_regular_ngon(radius, 8)
    }
}

fn bounding_rect_for_points<'a>(points: impl Iterator<Item = &'a Vec2>) -> Rect {
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
        min: Vec2::new(x_min, y_min),
        max: Vec2::new(x_max, y_max),
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

/// The input must not be empty.
/// No edge can cross any other edge, whether it is on the same polygon or not.
/// Each vertex must be part of exactly two edges. Polygons cannot 'share' vertices with each other.
/// Each vertex must be distinct - no vertex can have x and y coordinates that both compare equal to another vertex's.
#[derive(Debug)]
pub struct InvalidInput;

impl Display for InvalidInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid polygon input")
    }
}

impl Error for InvalidInput { }

impl<T: Error> From<TriangulationError<T>> for InvalidInput {
    fn from(value: TriangulationError<T>) -> Self {
        match value {
            TriangulationError::TrapezoidationError(_) => panic!("Failed to triangulate: {}", value),
            TriangulationError::NoVertices => Self,
            TriangulationError::InternalError(_) => Self,
            TriangulationError::FanBuilder(_) => panic!("Failed to triangulate: {}", value),
            _ => panic!("Failed to triangulate: {}", value),
        }
    }
}

impl TryFrom<Polygon> for Mesh {

    type Error = InvalidInput;

    fn try_from(polygon: Polygon) -> Result<Self, Self::Error> {

        if polygon.points.len() < 3 {
            return Err(InvalidInput);
        }

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(polygon.points.len());
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(polygon.points.len());
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(polygon.points.len());

        // The domain is needed for UV mapping. The domain tells us how to transform all points to optimally fit the 0-1 range.
        let domain = bounding_rect_for_points(polygon.points.iter());

        // Add the vertices
        for v in &polygon.points {
            positions.push([v.x, 0.0, v.y]);
            normals.push(Vec3::Y.to_array());

            // Transform the polygon domain to the 0-1 UV domain.
            let u = (v.x - domain.min.x) / (domain.max.x - domain.min.x);
            let v = (v.y - domain.min.y) / (domain.max.y - domain.min.y);
            uvs.push([u, v]);
        }

        // Triangulate to obtain the indices
        // This library is terrible to use. The heck is that initializer object. And this trait madness.
        let polygons = polygon
            .points
            .into_iter()
            .map(|v| Vec2f(v))
            .collect::<Vec<Vec2f>>();
        let mut output = Vec::<[usize; 3]>::new();
        let format = IndexedListFormat::new(&mut output).into_fan_format();
        triangulate::Polygon::triangulate(&polygons, format)?;
        let indices = output.into_iter()
            .map(|[a, b, c]| [c, b, a])
            .flatten()
            .map(|v| v as u32)
            .collect();

        // Put the mesh together
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        Ok(mesh)
    }
}
