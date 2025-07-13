// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - Vector.rs

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vector
{
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector
{
    pub fn raw(&self) -> [f32; 4]
    {
        return [self.x, self.y, self.z, self.w];
    }

    pub fn x(&self) -> f32
    {
        return self.x;
    }

    pub fn y(&self) -> f32
    {
        return self.y;
    }

    pub fn z(&self) -> f32
    {
        return self.z;
    }

    pub fn w(&self) -> f32
    {
        return self.w;
    }

    pub fn set_w(&mut self, w: f32)
    {
        self.w = w;
    }

    pub fn r(&self) -> f32
    {
        return self.x;
    }

    pub fn g(&self) -> f32
    {
        return self.y;
    }

    pub fn b(&self) -> f32
    {
        return self.z;
    }

    pub fn a(&self) -> f32
    {
        return self.w;
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector
    {
        return Vector{ x: x, y: y, z: z, w: w };
    }

    pub fn zero() -> Vector
    {
        return Vector{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    }

    pub fn one() -> Vector
    {
        return Vector{ x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    }

    pub fn broadcast(scalar: f32) -> Vector
    {
        return Vector{ x: scalar, y: scalar, z: scalar, w: scalar };
    }

    pub fn add(vec1: &Vector, vec2: &Vector) -> Vector
    { 
        return Vector{ x: vec1.x + vec2.x, y: vec1.y + vec2.y, z: vec1.z + vec2.z, w: vec1.w + vec2.w };
    }

    pub fn sub(vec1: &Vector, vec2: &Vector) -> Vector
    {
        return Vector{ x: vec1.x - vec2.x, y: vec1.y - vec2.y, z: vec1.z - vec2.z, w: vec1.w - vec2.w };
    }

    pub fn mul(vec1: &Vector, vec2: &Vector) -> Vector
    {
        return Vector{ x: vec1.x * vec2.x, y: vec1.y * vec2.y, z: vec1.z * vec2.z, w: vec1.w * vec2.w };
    }

    pub fn scale(vec1: &Vector, scalar: f32) -> Vector
    {
        return Vector{ x: vec1.x * scalar, y: vec1.y * scalar, z: vec1.z * scalar, w: vec1.w * scalar };
    }

    pub fn dot(vec1: &Vector, vec2: &Vector) -> f32
    {
        // w excluded
        return vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
    }

    pub fn cross(vec1: &Vector, vec2: &Vector) -> Vector
    {
        return Vector{ x: vec1.y * vec2.z - vec1.z * vec2.y, y: vec1.z * vec2.x - vec1.x * vec2.z, z: vec1.x * vec2.y - vec1.y * vec2.x, w: 0.0 };
    }

    pub fn length2(vec1: &Vector) -> f32
    {
        return Vector::dot(vec1, vec1);
    }

    pub fn length(vec1: &Vector) -> f32
    {
        return f32::sqrt(Vector::length2(vec1));
    }

    pub fn normalize(vec1: &Vector) -> Vector
    {
        let magnitude = Vector::length(vec1);

        if magnitude == 0.0
        {
            return Vector::zero();
        }

        return Vector::scale(vec1, 1.0 / magnitude);
    }

    pub fn lerp(vec1: &Vector, vec2: &Vector, t: f32) -> Vector
    {
        return Vector::add(vec1, &Vector::scale(&Vector::sub(vec2, vec1), t));
    }

    pub fn reflect(vec1: &Vector, normal: &Vector) -> Vector
    {
        let dot_product = Vector::dot(vec1, normal);
        return Vector::sub(vec1, &Vector::scale(normal, 2.0 * dot_product));
    }

    // TODO: refraction
}