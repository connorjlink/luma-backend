// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - geometry.rs

use crate::vector;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex
{
    position: vector::Vector,
    normal: vector::Vector,
    uv: vector::Vector, // used for color for now, texture mapping later
}

impl Vertex
{
    pub fn vertex_attributes() -> Vec<wgpu::VertexAttribute>
    {
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4].to_vec()
    }

    pub fn description(attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout
    {
        wgpu::VertexBufferLayout
        {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Block
{
    vertices: [Vertex; 8],
    indices: [u32; 36], // 6 faces, 2 triangles per face, 3 vertices per triangle
}
