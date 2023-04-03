use bevy::prelude::Vec3;

// When indexing a mesh we commonly find flat (occupying a 2 dimensional subspace) trapezes.
#[derive(Copy, Clone)]
pub(crate) struct FlatTrapezeIndices {
    pub lower_left: u32,
    pub upper_left: u32,
    pub lower_right: u32,
    pub upper_right: u32,
}

impl FlatTrapezeIndices {

    // Triangulate the trapeze
    pub fn generate_triangles(&self, indices: &mut Vec<u32>) {
        indices.push(self.upper_left);
        indices.push(self.upper_right);
        indices.push(self.lower_left);
        indices.push(self.upper_right);
        indices.push(self.lower_right);
        indices.push(self.lower_left);
    }
}

pub(crate) struct Extent {
    min: Vec3,
    max: Vec3,
}

impl Extent {
    pub fn new() -> Self {
        Extent {
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    pub fn extend_to_include(&mut self, v: Vec3) {
        // unwrap: we know the size of this array statically
        self.min.x = f32::min(self.min.x, v.x);
        self.min.y = f32::min(self.min.y, v.y);
        self.min.z = f32::min(self.min.z, v.z);
        self.max.x = f32::max(self.max.x, v.x);
        self.max.y = f32::max(self.max.y, v.y);
        self.max.z = f32::max(self.max.z, v.z);
    }

    pub fn lengths(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn center(&self) -> Vec3 {
        self.min + (self.max - self.min) / 2.0
    }
}