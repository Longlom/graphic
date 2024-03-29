use core::fmt;
use image::{Pixel, Rgb, RgbImage};
use std::path::Path;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn swap(a: &mut Point, b: &mut Point) {
        let _ = std::mem::swap(a, b);
    }
}

const CANVAS_WIDTH: i32 = 1000;
const CANVAS_HEIGHT: i32 = 1000;
const BACKGROUND_COLOR: Rgb<u8> = Rgb([255, 255, 255]);

fn put_pixel(canvas: &mut RgbImage, color: Rgb<u8>, coord: Point) {
    let y_offset = CANVAS_HEIGHT / 2;
    let x_offset = CANVAS_WIDTH / 2;

    if coord.x < -x_offset || coord.x > x_offset || coord.y < -y_offset || coord.y > y_offset {
        println!("Point - {:?} is out of bound ", coord);
        return;
    }

    canvas.put_pixel(
        (coord.x + x_offset) as u32,
        (coord.y + y_offset) as u32,
        color,
    );
}

/// Creates a Vec of dependant values d, d = f(i).
/// i - independant variables.
/// works with i32
fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<i32> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut values: Vec<i32> = vec![];
    let a = ((d1 - d0) as f32) / ((i1 - i0) as f32);
    let mut d = d0 as f32;

    for _i in i0..=i1 {
        values.push(d.round() as i32);

        d = d + a;
    }

    return values;
}

fn interpolate_f32(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut values: Vec<f32> = vec![];
    let a = (d1 - d0) / ((i1 - i0) as f32);
    let mut d = d0;

    for _i in i0..=i1 {
        values.push(d);

        d = d + a;
    }

    return values;
}

fn draw_line(
    canvas: &mut RgbImage,
    mut point_a: &mut Point,
    mut point_b: &mut Point,
    color: Rgb<u8>,
) {
    let dx = point_b.x - point_a.x;
    let dy = point_b.y - point_a.y;

    if dx.abs() > dy.abs() {
        //line is horizontalish
        if point_a.x > point_b.x {
            Point::swap(&mut point_a, &mut point_b);
        }

        let ys = interpolate(point_a.x, point_a.y, point_b.x, point_b.y);

        for x in point_a.x..point_b.x {
            let point = Point::new(x, ys[(x - point_a.x) as usize]);
            put_pixel(canvas, color, point);
        }
    } else {
        // Line is vertical-ish
        if point_a.y > point_b.y {
            Point::swap(&mut point_a, &mut point_b);
        }

        let xs = interpolate(point_a.y, point_a.x, point_b.y, point_b.x);

        for y in point_a.y..point_b.y {
            let point = Point::new(xs[(y - point_a.y) as usize], y);
            put_pixel(canvas, color, point);
        }
    }
}

fn draw_wireframe_triangle(
    p0: &mut Point,
    p1: &mut Point,
    p2: &mut Point,
    color: Rgb<u8>,
    canvas: &mut RgbImage,
) {
    draw_line(canvas, p0, p1, color);
    draw_line(canvas, p1, p2, color);
    draw_line(canvas, p2, p0, color);
}

fn draw_filled_triangle(
    p0: &mut Point,
    p1: &mut Point,
    p2: &mut Point,
    color: Rgb<u8>,
    canvas: &mut RgbImage,
) {
    if p1.y < p0.y {
        Point::swap(p1, p0);
    }

    if p2.y < p0.y {
        Point::swap(p2, p0);
    }

    if p2.y < p1.y {
        Point::swap(p2, p1);
    }

    let mut x01 = interpolate(p0.y, p0.x, p1.y, p1.x);
    let mut h01 = interpolate_f32(p0.y, 0., p1.y, 0.4);

    let mut x12 = interpolate(p1.y, p1.x, p2.y, p2.x);
    let mut h12 = interpolate_f32(p1.y, 0.4, p2.y, 0.9);

    let x02 = interpolate(p0.y, p0.x, p2.y, p2.x);
    let mut h02 = interpolate_f32(p0.y, 0., p2.y, 0.9);

    x01.pop().unwrap();
    x01.append(&mut x12);

    h01.pop().unwrap();
    h01.append(&mut h12);

    let m = ((x02.len() as f32) / 2.).floor() as usize;

    let x_left;
    let x_right;

    let h_left;
    let h_right;

    if x02[m] < x01[m] {
        x_left = x02;
        h_left = h02;

        x_right = x01;
        h_right = h01;
    } else {
        x_left = x01;
        h_left = h01;

        x_right = x02;
        h_right = h02;
    }

    for y in p0.y..=p2.y {
        let x_l = x_left[(y - p0.y) as usize];
        let x_r = x_right[(y - p0.y) as usize];

        let h_segment = interpolate_f32(
            x_l,
            h_left[(y - p0.y) as usize],
            x_r,
            h_right[(y - p0.y) as usize],
        );

        for x in x_l..x_r {
            let mut color = color.clone();
            color.apply(|x_in| ((x_in as f32) * h_segment[(x - x_l) as usize]).round() as u8);
            put_pixel(canvas, color, Point::new(x, y))
        }
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

    // draw_line(
    //     &mut canvas,
    //     &mut Point { x: 1300, y: 0 },
    //     &mut Point { x: 0, y: 1300 },
    //     Rgb([0, 0, 0]),
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut Point { x: 0, y: 0 },
    //     &mut Point { x: 450, y: 500 },
    //     Rgb([0, 0, 0]),
    // );

    draw_filled_triangle(
        &mut Point { x: 0, y: 0 },
        &mut Point { x: -40, y: 300 },
        &mut Point { x: 300, y: 100 },
        Rgb([0, 0, 0]),
        &mut canvas,
    );

    canvas.save(path).unwrap();
}
