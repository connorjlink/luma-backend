// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - buffer.rs

use std::marker::PhantomData;

pub struct Buffer<T>
{
    usage: BufferUsage,
    id: u32, // GLuint
    gl_type: u32, // GLenum
    attribute_id: u32,
    phantom: PhantomData<T>,
}

impl<T> Buffer<T> 
{
    pub fn new(gl_type: u32, usage: BufferUsage, data: &[T], hint: u32) -> Self
    {
        let mut id: u32 = 0;
        unsafe 
        {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl_type, id);
            let stride = std::mem::size_of::<T>();
            let size = data.len() * stride;
            gl::BufferData
            (
                gl_type,
                size as isize,
                data.as_ptr() as *const _,
                hint,
            );
        }
        Buffer
        {
            usage,
            id,
            gl_type,
            attribute_id: 0,
            phantom: PhantomData,
        }
    }

    pub fn bind(&self) 
    {
        unsafe 
        {
            gl::BindBuffer(self.gl_type, self.id);
        }
    }

    pub fn add_attribute(&mut self, element_count: u32, element_type: u32, stride: u32, offset: usize) 
    {
        unsafe 
        {
            gl::VertexAttribPointer
            (
                self.attribute_id,
                element_count as i32,
                element_type,
                gl::FALSE,
                stride as i32,
                offset as *const _,
            );
            gl::EnableVertexAttribArray(self.attribute_id);
        }
        self.attribute_id += 1;
    }

    pub fn base(&self) 
    {
        unsafe 
        {
            gl::BindBufferBase(self.gl_type, self.attribute_id, self.id);
        }
    }
}
