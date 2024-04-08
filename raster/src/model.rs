use crate::{Color, Matrix, VectorPoint};

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
    pub transform_matrix: Matrix,
}

impl Model {
    pub fn new(
        name: ModelName,
        vertices: Vec<VectorPoint>,
        triangles: Vec<Triangle>,
        transform: Transform,
    ) -> Self {
        let transform_matrix = Matrix::new_translation_matrix(transform.translation)
            * (Matrix::new_Y_rotation_matrix(transform.rotation)
                * Matrix::new_scale_matrix(transform.scale));
        Self {
            name,
            vertices,
            triangles,
            transform,
            transform_matrix,
        }
    }
}
