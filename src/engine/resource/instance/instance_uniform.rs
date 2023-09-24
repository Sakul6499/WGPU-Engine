use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceUniform {
    pub model_space_matrix: [[f32; 4]; 4],
}
