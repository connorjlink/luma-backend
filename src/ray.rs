// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - ray.rs

use crate::vector::*;

#[derive(Clone)]
#[derive(Copy)]
pub struct Ray
{
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray
{
    pub fn new(origin: Vector, direction: Vector) -> Ray
    {
        return Ray{ origin: origin, direction: direction };
    }

    pub fn zero() -> Ray
    {
        return Ray{ origin: Vector::zero(), direction: Vector::zero() };
    }
}