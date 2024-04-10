use std::fmt;

use image::Rgb;

use crate::{matrix::Matrix, vector_point::VectorPoint, HomogenousVectorPoint};

pub type Color = Rgb<u8>;

pub const THRESHOLD_CANVAS: i32 = 10;
pub const CANVAS_WIDTH: i32 = 1500;
pub const CANVAS_HEIGHT: i32 = 1500;

pub const VIEWPORT_SIZE: f32 = 5.0;
pub const PROJECTION_PLANE_Z: f32 = 4.0;

pub const BACKGROUND_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

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
        let res = Point {
            x: (x * (CANVAS_WIDTH as f32) / VIEWPORT_SIZE).round() as i32,
            y: (y * (CANVAS_HEIGHT as f32) / VIEWPORT_SIZE).round() as i32,
        };
        res
    }
}

pub struct Plane {
    pub normal: VectorPoint,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: VectorPoint, distance: f32) -> Self {
        Self { normal, distance }
    }
}

pub struct Camera {
    pub position: VectorPoint,
    pub orientation: Matrix,
    pub clipping_planes: Vec<Plane>,
}

impl Camera {
    pub fn new(position: VectorPoint, orientation: Matrix, clipping_planes: Vec<Plane>) -> Self {
        Self {
            position,
            orientation,
            clipping_planes,
        }
    }
}

pub fn dot(v1: &HomogenousVectorPoint, v2: &HomogenousVectorPoint) -> f32 {
    v1.values[0] * v2.values[0] + v1.values[1] * v2.values[1] + v1.values[2] * v2.values[2]
}
