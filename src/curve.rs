use std::ops::{Deref, Sub};
use bevy::prelude::{Mesh, Quat, Vec2, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::MeshData;
use crate::util::{Extent, FlatTrapezeIndices};

/// A curve is some math function in 3d.
/// It is defined and sampled in the domain [0, 1].
/// The parameter t is the offset in that [0, 1] range which is sampled uniformly by the library to create frames.
pub trait CurveFunction {

    /// Evaluate the curve at some point along it.
    fn eval_at(&self, t: f32) -> Vec3;

    /// Calculate a unit tangent at a specific point on the curve.
    /// By default it will take two close points and use their difference to construct the tangent.
    fn tangent_at(&self, t: f32) -> Vec3 {
        const DELTA: f32 = 0.0001;

        let t0 = t - DELTA;
        let t1 = t + DELTA;
        let v0 = self.eval_at(t0);
        let v1 = self.eval_at(t1);

        (v1 - v0).normalize()
    }
}

/// Default curve implementation. It's a straight line up (y+).
/// This is mainly used as a fallback and is thus not public.
/// Users are expected to bring their own curve implementations.
struct DefaultCurve;

impl CurveFunction for DefaultCurve {
    fn eval_at(&self, t: f32) -> Vec3 {
        assert!(t >= 0.0);
        assert!(t <= 1.0);
        Vec3::new(0.0, t, 0.0)
    }

    fn tangent_at(&self, _: f32) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
}

/// A curve is a shape that follows a curve function.
/// It can be 3 things: A tube, a line, or a ribbon.
/// To create a ribbon simple set <3 radial segments.
/// To create a line set the radius to 0.
/// Everything else is interpreted as a curve.
pub struct Curve {
    /// Radius of the tube. Set to 0 for a line.
    pub radius: f32,
    /// Underlying curve function to track
    pub curve: Box<dyn CurveFunction>,
    /// Number of samples taken from the curve function
    pub length_segments: u32,
    /// Number of segments around the tube. Set to 1 or 2 to create a ribbon (1 single-sided, 2 double-sided).
    pub radial_segments: u32,
    /// The circumference around the tube. If this is less than 2pi the tube will be open.
    pub radial_circumference: f32,
    /// The offset in radians on the tube radius.
    /// For ribbons this specifies the orientation of the ribbon against the function line.
    pub radial_offset: f32,
}

impl Default for Curve {
    fn default() -> Self {
        Curve {
            radius: 0.05,
            curve: Box::new(DefaultCurve), // straight line
            length_segments: 64,
            radial_segments: 64,
            radial_circumference: std::f32::consts::TAU,
            radial_offset: 0.0,
        }
    }
}

struct FrenetSerretFrame {
    origin: Vec3,
    tangent: Vec3,
    normal: Vec3,
    binormal: Vec3,
}

fn initial_normal(tangent: Vec3) -> Vec3 {

    // Select initial normal in the direction of the minimum component of the tangent
    let mut min = f32::MAX;
    let tx = tangent.x.abs();
    let ty = tangent.y.abs();
    let tz = tangent.z.abs();

    let mut normal = Vec3::new(0.0, 0.0, 0.0);

    if tx <= min {
        min = tx;
        normal = Vec3::new(1.0, 0.0, 0.0);
    }
    if ty <= min {
        min = ty;
        normal = Vec3::new(0.0, 1.0, 0.0);
    }
    if tz <= min {
        normal = Vec3::new(0.0, 0.0, 1.0);
    }

    normal
}

fn initial_frame(curve: &dyn CurveFunction) -> FrenetSerretFrame {

    let origin = curve.eval_at(0.0);
    let tangent = curve.tangent_at(0.0);
    let normal = initial_normal(tangent);
    let v = tangent.cross(tangent.cross(normal).normalize());

    FrenetSerretFrame {
        origin,
        tangent,
        normal: v,
        binormal: tangent.cross(v),
    }
}

fn calculate_frames(curve: &dyn CurveFunction, num_frames: u32) -> Vec<FrenetSerretFrame> {

    let mut out = Vec::with_capacity(num_frames as usize);
    let step = 1.0 / (num_frames - 1) as f32;
    
    // First frame is different
    out.push(initial_frame(curve));

    // Calculate a smoothly shifting coordinate frame for each segment point
    for i in 1..num_frames {

        let t = step * i as f32;
        let prev_frame: &FrenetSerretFrame = out.get(i as usize - 1).unwrap(); // unwrap: i starts at 1

        let mut cur_frame = FrenetSerretFrame {
            origin: curve.eval_at(t),
            tangent: curve.tangent_at(t),
            normal: prev_frame.normal,
            binormal: prev_frame.binormal,
        };

        let mut v = prev_frame.tangent.cross(cur_frame.tangent);
        if v.length() > f32::EPSILON {
            v = v.normalize();
            let angle = prev_frame.tangent.dot(cur_frame.tangent);
            let angle = angle.clamp(-1.0, 1.0);
            let theta = f32::acos(angle);
            let rot = Quat::from_axis_angle(v, theta);
            cur_frame.normal = rot.mul_vec3(cur_frame.normal);
        }

        cur_frame.binormal = cur_frame.tangent.cross(cur_frame.normal);

        out.push(cur_frame);
    }

    // If the curve is closed, make the frames line up
    let start_end_distance = curve.eval_at(0.0).sub(curve.eval_at(1.0)).length();
    if start_end_distance <= 2.0 * f32::EPSILON {

        let first_frame = out.get(0).unwrap(); // unwrap: We have >= 1 segment
        let last_frame = out.last().unwrap(); // unwrap: We have >= 1 segment

        // Post-process the frames
        let discrepancy_theta = {
            let t = first_frame.normal.dot(last_frame.normal)
                .clamp(-1.0, 1.0)
                .acos() / (num_frames - 1) as f32;
            if first_frame.tangent.dot(first_frame.normal.cross(last_frame.normal)) > 0.0 {
                -t
            }
            else {
                t
            }
        };

        // Rotate each frame a little to make them line up
        for (idx, frame) in out.iter_mut().skip(1).enumerate() {
            let rot = Quat::from_axis_angle(frame.tangent, discrepancy_theta * idx as f32);
            frame.normal = rot.mul_vec3(frame.normal);
            frame.binormal = frame.tangent.cross(frame.normal);
        }
    }

    out
}

fn normalize_frames(frames: &mut [FrenetSerretFrame]) {
    let mut extent = Extent::new();
    for frame in frames.iter() {
        extent.extend_to_include(frame.origin);
    }
    let center = extent.center();
    let lengths = extent.lengths().to_array();
    let scale = 1.0 / lengths.iter()
        .fold(f32::MIN, |a, b| f32::max(a, f32::abs(*b)));
    for frame in frames.iter_mut() {
        frame.origin -= center;
        frame.origin *= scale;
    }
}

fn add_tube_segment(mesh: &mut MeshData, frame: &FrenetSerretFrame, tube: &Curve, index: usize) {

    let angle_step = tube.radial_circumference / tube.radial_segments as f32;

    for i in 0..=tube.radial_segments {
        let theta = angle_step * i as f32 + tube.radial_offset;
        let sin = theta.sin();
        let cos = -theta.cos();

        let normal = Vec3::normalize(cos * frame.normal + sin * frame.binormal);
        let position = frame.origin + tube.radius * normal;
        let uv = Vec2::new(
            index as f32 / tube.length_segments as f32,
            i as f32 / tube.radial_segments as f32
        );

        mesh.normals.push(normal);
        mesh.positions.push(position);
        mesh.uvs.push(uv);
    }
}

fn add_ribbon_segment(mesh: &mut MeshData, frame: &FrenetSerretFrame, tube: &Curve, index: usize) {

    let theta = tube.radial_offset + std::f32::consts::FRAC_PI_2;
    let sin = theta.sin();
    let cos = -theta.cos();
    let base = Vec3::normalize(cos * frame.normal + sin * frame.binormal);

    // Front
    let front_normal = frame.tangent.cross(base);
    mesh.normals.push(front_normal);
    mesh.normals.push(front_normal);
    mesh.positions.push(frame.origin + tube.radius * base);
    mesh.positions.push(frame.origin + tube.radius * -base);
    mesh.uvs.push(Vec2::new(
        index as f32 / tube.length_segments as f32,
        0.0
    ));
    mesh.uvs.push(Vec2::new(
        index as f32 / tube.length_segments as f32,
        1.0
    ));

    // Back
    if tube.radial_segments == 2 {
        mesh.normals.push(-front_normal);
        mesh.normals.push(-front_normal);
        mesh.positions.push(frame.origin + tube.radius * -base);
        mesh.positions.push(frame.origin + tube.radius * base);
        mesh.uvs.push(Vec2::new(
            index as f32 / tube.length_segments as f32,
            0.0
        ));
        mesh.uvs.push(Vec2::new(
            index as f32 / tube.length_segments as f32,
            1.0
        ));
    }
}

// Calculate the bounding box of this mesh and then shrink the mesh to fit into the unit box
fn normalize_positions(positions: &mut [Vec3]) {

    let mut extent = Extent::new();
    for point in positions.iter() {
        extent.extend_to_include(*point);
    }
    let center = extent.center();
    let lengths = extent.lengths().to_array();
    let scale = 1.0 / lengths.iter()
        .fold(f32::MIN, |a, b| f32::max(a, f32::abs(*b)));
    for point in positions.iter_mut() {
        *point -= center;
        *point *= scale;
    }
}

fn index_tube(mesh: &mut MeshData, tube: &Curve) {
    for j in 1..=tube.length_segments {
        for i in 1..=tube.radial_segments {

            let a = ( tube.radial_segments + 1 ) * ( j - 1 ) + ( i - 1 );
            let b = ( tube.radial_segments + 1 ) * j + ( i - 1 );
            let c = ( tube.radial_segments + 1 ) * j + i;
            let d = ( tube.radial_segments + 1 ) * ( j - 1 ) + i;

            // faces
            mesh.indices.push(a);
            mesh.indices.push(b);
            mesh.indices.push(d);
            mesh.indices.push(b);
            mesh.indices.push(c);
            mesh.indices.push(d);
        }
    }
}

fn index_ribbon(mesh: &mut MeshData, tube: &Curve) {
    for ls in 0..tube.length_segments {
        for rs in 0..tube.radial_segments {
            let indices = FlatTrapezeIndices {
                lower_left: 2 * tube.radial_segments * ls + 2 * rs,
                upper_left: 2 * tube.radial_segments * (ls + 1) + 2 * rs,
                lower_right: 2 * tube.radial_segments * ls + 2 * rs + 1,
                upper_right: 2 * tube.radial_segments * (ls + 1) + 2 * rs + 1,
            };
            indices.generate_triangles(&mut mesh.indices);
        }
    }
}

// The implementation of this algorithm is based on three.js.
// https://github.com/mrdoob/three.js
fn add_tube(mesh: &mut MeshData, tube: &Curve) {

    let mut frames = calculate_frames(tube.curve.deref(), tube.length_segments + 1);
    normalize_frames(frames.as_mut_slice());
    for (idx, frame) in frames.iter().enumerate() {
        if tube.radial_segments < 3 {
            add_ribbon_segment(mesh, frame, tube, idx);
        }
        else {
            add_tube_segment(mesh, frame, tube, idx);
        }
    }

    // Generate indices for the faces
    if tube.radial_segments < 3 {
        index_ribbon(mesh, tube);
    }
    else {
        index_tube(mesh, tube);
    }
}

fn make_line(tube: &Curve) -> Mesh {
    let mut m = Mesh::new(PrimitiveTopology::LineStrip);
    let mut positions = Vec::with_capacity(tube.length_segments as usize + 1);
    let step = 1.0 / tube.length_segments as f32;
    for i in 0..=tube.length_segments {
        let t = step * i as f32;
        let p = tube.curve.eval_at(t);
        positions.push(p);
    }
    normalize_positions(positions.as_mut_slice());
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    m
}

impl From<Curve> for Mesh {
    fn from(tube: Curve) -> Self {

        assert!(tube.length_segments > 0, "Must have at least one length segment");

        // Special case: Tube should be a line
        if tube.radius.abs() < f32::EPSILON || tube.radial_segments == 0 {
            return make_line(&tube);
        }

        assert!(tube.radial_segments > 0, "Must have at least one radial segment");
        assert!(tube.radial_offset >= 0.0 && tube.radial_offset <= std::f32::consts::TAU, "Radial offset must be in [0, 2pi]");
        assert!(tube.radial_circumference > 0.0 && tube.radial_circumference <= std::f32::consts::TAU, "Radial circumference must be in (0, 2pi]");

        let num_vertices = (tube.length_segments + 1) as usize * (tube.radial_segments + 1) as usize;
        let num_indices = tube.length_segments as usize * tube.radial_segments as usize * 6;
        let mut mesh = MeshData::new(num_vertices, num_indices);

        add_tube(&mut mesh, &tube);

        let mut m = Mesh::new(PrimitiveTopology::TriangleList);
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uvs);
        m.set_indices(Some(Indices::U32(mesh.indices)));
        m
    }
}