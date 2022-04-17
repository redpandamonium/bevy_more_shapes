
// When indexing a mesh we commonly find flat (occupying a 2 dimensional subspace) trapezes.
#[derive(Copy, Clone)]
pub struct FlatTrapezeIndices {
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