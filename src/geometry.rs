// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - geometry.rs

mod vector;

#[derive(Clone)]
#[derive(Copy)]
pub struct Vertex
{
    position: vector::Vector,
    normal: vector::Vector,
    uv: vector::Vector, // used for color for now, texture mapping later
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Block
{
    vertices: [Vertex; 8],
    indices: [u32; 36], // 6 faces, 2 triangles per face, 3 vertices per triangle
}
