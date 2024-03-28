use image::{Rgb, RgbImage};
use imageproc::point;
use std::{
    ops::{Div, Mul, Sub},
    path::Path,
};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// impl Sub for Point {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }

// impl Div for Point {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self::Output {

//     }
// }

type VectorPoint = (f32, f32, f32);

const CANVAS_WIDTH: i32 = 500;
const CANVAS_HEIGHT: i32 = 500;
const VIEWPORT_SIZE: f32 = 2.0;
const PROJECTION_PLANE_Z: f32 = 1.0;
const BACKGROUND_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

fn put_pixel(canvas: &mut RgbImage, color: Rgb<u8>, coord: Point) {
    if coord.x >= 0
        && coord.x < canvas.width() as i32
        && coord.y >= 0
        && coord.y < canvas.height() as i32
    {
        canvas.put_pixel(coord.x as u32, coord.y as u32, color);
    }
}

fn canvas_to_viewport(x: f32, y: f32) -> VectorPoint {
    (
        x * VIEWPORT_SIZE / CANVAS_WIDTH as f32,
        y * VIEWPORT_SIZE / CANVAS_HEIGHT as f32,
        PROJECTION_PLANE_Z,
    )
}

fn substract_vector(a: VectorPoint, b: VectorPoint) -> VectorPoint {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add_vector(a: VectorPoint, b: VectorPoint) -> VectorPoint {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn dot_number(a: VectorPoint, b: f32) -> VectorPoint {
    (a.0 * b, a.1 * b, a.2 * b)
}

fn dot_vector(a: VectorPoint, b: VectorPoint) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn length(a: VectorPoint) -> f32 {
    (a.0 * a.0 + a.1 * a.1 + a.2 * a.2).sqrt()
}

fn negate(a: VectorPoint) -> VectorPoint {
    (-a.0, -a.1, -a.2)
}

fn divide_number(a: VectorPoint, b: f32) -> VectorPoint {
    (a.0 / b, a.1 / b, a.2 / b)
}

fn draw_line(canvas: &mut RgbImage, mut point_a: Point, mut point_b: Point, color: Rgb<u8>) {
    if point_a.x > point_b.x {
      let temp = Point {..point_b};
      point_b = point_a;  
      point_a = temp;
    } 

    let y1 = point_b.y as f32;
    let y0 = point_a.y as f32;
    let x1 = point_b.x as f32;
    let x0 = point_a.x as f32;
    let a = (y1 - y0) / (x1 - x0);


    // let b = y0 - a * x0;

    let mut y = point_a.y;

    for x in point_a.x..point_b.x {
        // let y = (a * (x as f32) + b).round() as i32;
        put_pixel(canvas, color, Point { x, y });
        y = y + a.round() as i32;
    }
}

fn main() {
    let path = Path::new("./imgs/1_draw_line.png");

    let mut canvas = RgbImage::new(
        u32::try_from(CANVAS_WIDTH).unwrap(),
        u32::try_from(CANVAS_HEIGHT).unwrap(),
    );

    for (x, y, pix) in canvas.enumerate_pixels_mut() {
        pix.0 = BACKGROUND_COLOR.0;
    }

    // canvas.fill_with(f)

    draw_line(
        &mut canvas,
        Point { x: 0, y: 0 },
        Point { x: 1300, y: 1300 },
        Rgb([0, 0, 0]),
    );

    draw_line(
        &mut canvas,
        Point { x: 1300, y: 0 },
        Point { x: 0, y: 1300 },
        Rgb([0, 0, 0]),
    );

    draw_line(
        &mut canvas,
        Point { x: 100, y: 0 },
        Point { x: 60, y: 1300 },
        Rgb([0, 0, 0]),
    );

    canvas.save(path).unwrap();
}
