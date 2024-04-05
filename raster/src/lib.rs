use core::fmt;
use image::Rgb;
use std::ops;

pub type Color = Rgb<u8>;

pub const THRESHOLD_CANVAS: i32 = 10;
pub const CANVAS_WIDTH: i32 = 1500;
pub const CANVAS_HEIGHT: i32 = 1500;

pub const VIEWPORT_SIZE: f32 = 4.0;
pub const PROJECTION_PLANE_Z: f32 = 4.0;

pub const BACKGROUND_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

pub const MATRIX_DIM: usize = 4;

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn swap(a: &mut Point, b: &mut Point) {
        std::mem::swap(a, b);
    }

    pub fn viewport_to_canvas(x: f32, y: f32) -> Self {
        Point {
            x: (x * (CANVAS_WIDTH as f32) / VIEWPORT_SIZE).round() as i32,
            y: (y * (CANVAS_HEIGHT as f32) / VIEWPORT_SIZE).round() as i32,
        }
    }
}

pub struct Matrix {
    values: [[f32; MATRIX_DIM]; MATRIX_DIM],
}

impl Matrix {
    pub fn new_scale_matrix(scale: f32) -> Self {
        let mut res = Self::default();

        for i in 0..(MATRIX_DIM - 1) {
            res.values[i][i] = scale;
        }

        res
    }

    pub fn new_rotation_matrix(degree: i32) -> Self {
        let mut res = Self::default();
        let radian = (degree as f32).to_radians();
        res.values[0][0] = radian.cos();
        res.values[2][0] = -radian.sin();
        res.values[1][1] = 1.;
        res.values[0][2] = radian.sin();
        res.values[2][2] = radian.cos();

        res
    }

    pub fn new_translation_matrix(translate: VectorPoint) -> Self {
        let mut res = Self::default();

        res.values[0][MATRIX_DIM - 1] = translate.x;
        res.values[2][MATRIX_DIM - 1] = translate.y;
        res.values[3][MATRIX_DIM - 1] = translate.z;

        res.values[0][0] = 1.;
        res.values[1][1] = 1.;
        res.values[2][2] = 1.;

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

struct HomogenousVectorPoint {
    values: [f32; MATRIX_DIM],
}

impl HomogenousVectorPoint {
    fn new(v: &VectorPoint) -> Self {
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

        let matrix = Matrix::new_rotation_matrix(degree);

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

#[derive(Clone)]
pub struct Triangle {
    pub vertex: (usize, usize, usize),
    pub color: Color,
}

impl Triangle {
    pub fn new(vertex: (usize, usize, usize), color: Color) -> Self {
        Self { vertex, color }
    }
}

pub enum ModelName {
    CUBE,
}

#[derive(Clone)]
pub struct Transform {
    pub scale: f32,
    pub rotation: i32,
    pub translation: VectorPoint,
}

impl Transform {
    pub fn new(scale: f32, rotation: i32, translation: VectorPoint) -> Self {
        Self {
            scale,
            rotation,
            translation,
        }
    }
}

pub struct Model {
    pub name: ModelName,
    pub vertices: Vec<VectorPoint>,
    pub triangles: Vec<Triangle>,
    pub transform: Transform,
}

impl Model {
    pub fn new(
        name: ModelName,
        vertices: Vec<VectorPoint>,
        triangles: Vec<Triangle>,
        transform: Transform,
    ) -> Self {
        Self {
            name,
            vertices,
            triangles,
            transform,
        }
    }
}
