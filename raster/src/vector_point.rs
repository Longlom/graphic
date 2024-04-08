use std::ops;

use crate::matrix::{Matrix, MATRIX_DIM};

pub struct HomogenousVectorPoint {
    pub(crate) values: [f32; MATRIX_DIM],
}

impl HomogenousVectorPoint {
    pub fn new(v: &VectorPoint) -> Self {
        Self {
            values: [v.x, v.y, v.z, 1.],
        }
    }
}

#[derive(Clone, Copy)]
pub struct VectorPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VectorPoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn scale(&self, scale: f32) -> Self {
        let p = HomogenousVectorPoint::new(self);
        let matrix = Matrix::new_scale_matrix(scale);

        let res = p * matrix;

        Self {
            x: res.values[0],
            y: res.values[1],
            z: res.values[2],
        }
    }

    pub fn rotate(&self, degree: i32) -> Self {
        let p = HomogenousVectorPoint::new(self);

        let matrix = Matrix::new_Y_rotation_matrix(degree);

        let res = matrix * p;

        Self {
            x: res.values[0],
            y: res.values[1],
            z: res.values[2],
        }
    }

    pub fn translate(&self, translation: Self) -> Self {
        let p = HomogenousVectorPoint::new(self);

        let matrix = Matrix::new_translation_matrix(translation);

        let res = matrix * p;

        VectorPoint::from(res)
    }
}

impl From<HomogenousVectorPoint> for VectorPoint {
    fn from(value: HomogenousVectorPoint) -> Self {
        Self {
            x: value.values[0],
            y: value.values[1],
            z: value.values[2],
        }
    }
}

impl ops::Add for VectorPoint {
    type Output = VectorPoint;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f32> for VectorPoint {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Mul<VectorPoint> for f32 {
    type Output = VectorPoint;

    fn mul(self, rhs: VectorPoint) -> Self::Output {
        VectorPoint {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl ops::Mul<Matrix> for HomogenousVectorPoint {
    type Output = HomogenousVectorPoint;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut res = HomogenousVectorPoint {
            values: [0., 0., 0., 0.],
        };

        for i in 0..MATRIX_DIM {
            let mut value = 0.;
            for j in 0..MATRIX_DIM {
                value += self.values[j] * rhs.values[j][i];
            }
            res.values[i] = value;
        }

        res
    }
}
