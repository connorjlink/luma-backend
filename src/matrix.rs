// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - Matrix.rs

use crate::vector::*;

#[derive(Clone)]
pub struct Matrix
{
    m: [[f32; 4]; 4],
}

impl Matrix
{
    pub fn new(m: [[f32; 4]; 4]) -> Matrix
    {
        return Matrix{ m: m };
    }

    pub fn null() -> Matrix
    {
        return Matrix{ m: [[0.0; 4]; 4] };
    }

    pub fn identity() -> Matrix
    {
        return Matrix{ m: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
    }

    pub fn scale(mat1: &Matrix, scalar: f32) -> Matrix
    {
        let mut result = Matrix::null();
        
        for i in 0..4
        {
            for j in 0..4
            {
                result.m[i][j] = mat1.m[i][j] * scalar;
            }
        }

        return result;
    }

    pub fn add(mat1: &Matrix, mat2: &Matrix) -> Matrix
    {
        let mut result = Matrix::null();
        
        for i in 0..4
        {
            for j in 0..4
            {
                result.m[i][j] = mat1.m[i][j] + mat2.m[i][j];
            }
        }

        return result;
    }

    pub fn sub(mat1: &Matrix, mat2: &Matrix) -> Matrix
    {
        let mut result = Matrix::null();
        
        for i in 0..4
        {
            for j in 0..4
            {
                result.m[i][j] = mat1.m[i][j] - mat2.m[i][j];
            }
        }

        return result;
    }

    pub fn mul(mat1: &Matrix, mat2: &Matrix) -> Matrix
    {
        let mut result = Matrix::null();
        
        for i in 0..4
        {
            for j in 0..4
            {
                for k in 0..4
                {
                    result.m[i][j] += mat1.m[i][k] * mat2.m[k][j];
                }
            }
        }

        return result;
    }

    pub fn apply(mat1: &Matrix, vec1: &Vector) -> Vector
    {
        let mut result = Vector::zero();
        
        for i in 0..4
        {
            for j in 0..4
            {
                let scaled = Vector::scale(&vec1, mat1.m[i][j]);
                result = Vector::add(&result, &scaled);
            }
        }

        return result;
    }

    pub fn transpose(mat1: &Matrix) -> Matrix
    {
        let mut result = Matrix::null();
        
        for i in 0..4
        {
            for j in 0..4
            {
                result.m[i][j] = mat1.m[j][i];
            }
        }

        return result;
    }

    pub fn determinant(mat1: &Matrix) -> f32
    {
        let m = &mat1.m;

        // laplace expansion
        let det = m[0][0] * (
            m[1][1] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
            m[1][2] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) +
            m[1][3] * (m[2][1] * m[3][2] - m[2][2] * m[3][1])
        ) - m[0][1] * (
            m[1][0] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
            m[1][2] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
            m[1][3] * (m[2][0] * m[3][2] - m[2][2] * m[3][0])
        ) + m[0][2] * (
            m[1][0] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) -
            m[1][1] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
            m[1][3] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])
        ) - m[0][3] * (
            m[1][0] * (m[2][1] * m[3][2] - m[2][2] * m[3][1]) -
            m[1][1] * (m[2][0] * m[3][2] - m[2][2] * m[3][0]) +
            m[1][2] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])
        );

        return det;
    }

    pub fn minors(mat1: &Matrix) -> Matrix
    {
        let mut result = Matrix::null();
        
        for i in 0..4
        {
            for j in 0..4
            {
                let mut submat = mat1.clone();

                for k in 0..4
                {
                    submat.m[i][k] = 1.0;
                    submat.m[k][j] = 1.0;
                }

                result.m[i][j] = Matrix::determinant(&submat);
            }
        }

        return result;
    }

    pub fn cofactors(mat1: &Matrix) -> Matrix
    {
        let mut result = Matrix::minors(mat1);
        
        for i in 0..4
        {
            for j in 0..4
            {
                if ((i + j) & 1) != 0
                {
                    result.m[i][j] *= -1.0;
                }
            }
        }

        return result;
    }

    pub fn adjugate(mat1: &Matrix) -> Matrix
    {
        let cofactors = Matrix::cofactors(mat1);
        return Matrix::transpose(&cofactors);
    }

    pub fn inverse_old(mat1: &Matrix) -> Matrix
    {
        let det = Matrix::determinant(mat1);

        if det == 0.0
        {
            panic!("Matrix is singular and cannot be inverted");
        }

        let adjugate = Matrix::adjugate(mat1);
        return Matrix::scale(&adjugate, 1.0 / det);
    }

    pub fn inverse(mat1: &Matrix) -> Matrix
    {
        let A2323 = mat1.m[2][2] * mat1.m[3][3] - mat1.m[2][3] * mat1.m[3][2];
		let A1323 = mat1.m[2][1] * mat1.m[3][3] - mat1.m[2][3] * mat1.m[3][1];
		let A1223 = mat1.m[2][1] * mat1.m[3][2] - mat1.m[2][2] * mat1.m[3][1];
		let A0323 = mat1.m[2][0] * mat1.m[3][3] - mat1.m[2][3] * mat1.m[3][0];
		let A0223 = mat1.m[2][0] * mat1.m[3][2] - mat1.m[2][2] * mat1.m[3][0];
		let A0123 = mat1.m[2][0] * mat1.m[3][1] - mat1.m[2][1] * mat1.m[3][0];
		let A2313 = mat1.m[1][2] * mat1.m[3][3] - mat1.m[1][3] * mat1.m[3][2];
		let A1313 = mat1.m[1][1] * mat1.m[3][3] - mat1.m[1][3] * mat1.m[3][1];
		let A1213 = mat1.m[1][1] * mat1.m[3][2] - mat1.m[1][2] * mat1.m[3][1];
		let A2312 = mat1.m[1][2] * mat1.m[2][3] - mat1.m[1][3] * mat1.m[2][2];
		let A1312 = mat1.m[1][1] * mat1.m[2][3] - mat1.m[1][3] * mat1.m[2][1];
		let A1212 = mat1.m[1][1] * mat1.m[2][2] - mat1.m[1][2] * mat1.m[2][1];
		let A0313 = mat1.m[1][0] * mat1.m[3][3] - mat1.m[1][3] * mat1.m[3][0];
		let A0213 = mat1.m[1][0] * mat1.m[3][2] - mat1.m[1][2] * mat1.m[3][0];
		let A0312 = mat1.m[1][0] * mat1.m[2][3] - mat1.m[1][3] * mat1.m[2][0];
		let A0212 = mat1.m[1][0] * mat1.m[2][2] - mat1.m[1][2] * mat1.m[2][0];
		let A0113 = mat1.m[1][0] * mat1.m[3][1] - mat1.m[1][1] * mat1.m[3][0];
		let A0112 = mat1.m[1][0] * mat1.m[2][1] - mat1.m[1][1] * mat1.m[2][0];

		let mut det = mat1.m[0][0] * (mat1.m[1][1] * A2323 - mat1.m[1][2] * A1323 + mat1.m[1][3] * A1223)
				         - mat1.m[0][1] * (mat1.m[1][0] * A2323 - mat1.m[1][2] * A0323 + mat1.m[1][3] * A0223)
				         + mat1.m[0][2] * (mat1.m[1][0] * A1323 - mat1.m[1][1] * A0323 + mat1.m[1][3] * A0123)
				         - mat1.m[0][3] * (mat1.m[1][0] * A1223 - mat1.m[1][1] * A0223 + mat1.m[1][2] * A0123);
		
		det = 1.0 / det;

		let mut out = Matrix::null();

		out.m[0][0] = det *  (mat1.m[1][1] * A2323 - mat1.m[1][2] * A1323 + mat1.m[1][3] * A1223);
		out.m[0][1] = det * -(mat1.m[0][1] * A2323 - mat1.m[0][2] * A1323 + mat1.m[0][3] * A1223);
		out.m[0][2] = det *  (mat1.m[0][1] * A2313 - mat1.m[0][2] * A1313 + mat1.m[0][3] * A1213);
		out.m[0][3] = det * -(mat1.m[0][1] * A2312 - mat1.m[0][2] * A1312 + mat1.m[0][3] * A1212);
		out.m[1][0] = det * -(mat1.m[1][0] * A2323 - mat1.m[1][2] * A0323 + mat1.m[1][3] * A0223);
		out.m[1][1] = det *  (mat1.m[0][0] * A2323 - mat1.m[0][2] * A0323 + mat1.m[0][3] * A0223);
		out.m[1][2] = det * -(mat1.m[0][0] * A2313 - mat1.m[0][2] * A0313 + mat1.m[0][3] * A0213);
		out.m[1][3] = det *  (mat1.m[0][0] * A2312 - mat1.m[0][2] * A0312 + mat1.m[0][3] * A0212);
		out.m[2][0] = det *  (mat1.m[1][0] * A1323 - mat1.m[1][1] * A0323 + mat1.m[1][3] * A0123);
		out.m[2][1] = det * -(mat1.m[0][0] * A1323 - mat1.m[0][1] * A0323 + mat1.m[0][3] * A0123);
		out.m[2][2] = det *  (mat1.m[0][0] * A1313 - mat1.m[0][1] * A0313 + mat1.m[0][3] * A0113);
		out.m[2][3] = det * -(mat1.m[0][0] * A1312 - mat1.m[0][1] * A0312 + mat1.m[0][3] * A0112);
		out.m[3][0] = det * -(mat1.m[1][0] * A1223 - mat1.m[1][1] * A0223 + mat1.m[1][2] * A0123);
		out.m[3][1] = det *  (mat1.m[0][0] * A1223 - mat1.m[0][1] * A0223 + mat1.m[0][2] * A0123);
		out.m[3][2] = det * -(mat1.m[0][0] * A1213 - mat1.m[0][1] * A0213 + mat1.m[0][2] * A0113);
		out.m[3][3] = det *  (mat1.m[0][0] * A1212 - mat1.m[0][1] * A0212 + mat1.m[0][2] * A0112);
	
		return out;

    }

    pub fn lookat(eye: &Vector, at: &Vector, up: &Vector) -> Matrix
    {
        // forward
        let f = Vector::normalize(&Vector::sub(at, eye));

        // right
        let r = Vector::normalize(&Vector::cross(up, &f));

        // up
        let u = Vector::cross(&f, &r);

        return Matrix
        {
            m: [
                [r.x(), u.x(), -f.x(), 0.0],
                [r.y(), u.y(), -f.y(), 0.0],
                [r.z(), u.z(), -f.z(), 0.0],
                [-Vector::dot(&r, eye), -Vector::dot(&u, eye), Vector::dot(&f, eye), 1.0],
            ],
        };
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix
    {
        let f = 1.0 / f32::tan(fov / 2.0);
        let range_inv = 1.0 / (near - far);

        return Matrix
        {
            m: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (near + far) * range_inv, -1.0],
                [0.0, 0.0, (2.0 * near * far) * range_inv, 0.0],
            ],
        };
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix
    {
        return Matrix
        {
            m: [
                [2.0 / (right - left), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (top - bottom), 0.0, 0.0],
                [0.0, 0.0, -2.0 / (far - near), 0.0],
                [-(right + left) / (right - left), -(top + bottom) / (top - bottom), -(far + near) / (far - near), 1.0],
            ],
        };
    }

}