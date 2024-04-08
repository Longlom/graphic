use std::ops;

use crate::vector_point::{HomogenousVectorPoint, VectorPoint};

pub const MATRIX_DIM: usize = 4;

#[derive(Clone)]

pub struct Matrix {
    pub values: [[f32; MATRIX_DIM]; MATRIX_DIM],
}

impl Matrix {
    pub fn new_scale_matrix(scale: f32) -> Self {
        let mut res = Self::default();

        for i in 0..(MATRIX_DIM - 1) {
            res.values[i][i] = scale;
        }

        res.values[MATRIX_DIM - 1][MATRIX_DIM - 1] = 1.;

        res
    }

    pub fn new_Y_rotation_matrix(degree: i32) -> Self {
        let mut res = Self::default();
        let radian = (degree as f32).to_radians();
        res.values[0][0] = radian.cos();
        res.values[2][0] = -radian.sin();
        res.values[1][1] = 1.;
        res.values[0][2] = radian.sin();
        res.values[2][2] = radian.cos();

        res.values[MATRIX_DIM - 1][MATRIX_DIM - 1] = 1.;

        res
    }

    pub fn new_translation_matrix(translate: VectorPoint) -> Self {
        let mut res = Self::default();

        res.values[0][MATRIX_DIM - 1] = translate.x;
        res.values[1][MATRIX_DIM - 1] = translate.y;
        res.values[2][MATRIX_DIM - 1] = translate.z;

        res.values[0][0] = 1.;
        res.values[1][1] = 1.;
        res.values[2][2] = 1.;

        res
    }

    pub fn transpose(&self) -> Self {
        let mut res = Matrix::zeroed();
        for i in 0..MATRIX_DIM {
            for j in 0..MATRIX_DIM {
                res.values[i][j] = self.values[j][i];
            }
        }

        res
    }

    pub fn zeroed() -> Self {
        Self {
            values: [[0.; MATRIX_DIM]; MATRIX_DIM],
        }
    }

    pub fn identity() -> Self {
        let mut res = Self::default();

        for i in 0..MATRIX_DIM {
            for j in 0..MATRIX_DIM {
                if i == j {
                    res.values[i][j] = 1.;
                }
            }
        }

        res
    }
}

impl Default for Matrix {
    fn default() -> Self {
        let mut res = Self {
            values: [[0.; MATRIX_DIM]; MATRIX_DIM],
        };

        res.values[MATRIX_DIM - 1][MATRIX_DIM - 1] = 1.;
        res
    }
}

impl ops::Mul<HomogenousVectorPoint> for Matrix {
    type Output = HomogenousVectorPoint;
    fn mul(self, rhs: HomogenousVectorPoint) -> Self::Output {
        let mut res = HomogenousVectorPoint {
            values: [0., 0., 0., 0.],
        };

        for i in 0..MATRIX_DIM {
            let mut value = 0.;
            for j in 0..MATRIX_DIM {
                value += self.values[i][j] * rhs.values[j];
            }
            res.values[i] = value;
        }

        res
    }
}

impl ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::zeroed();

        for i in 0..MATRIX_DIM {
            for j in 0..MATRIX_DIM {
                for k in 0..MATRIX_DIM {
                    res.values[i][j] += self.values[i][k] * rhs.values[k][j];
                }
            }
        }

        res
    }
}
