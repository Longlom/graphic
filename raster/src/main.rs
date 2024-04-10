use image::{Pixel, Rgb, RgbImage};
use std::path::Path;

use core::*;
use matrix::*;
use model::*;
use vector_point::*;

mod core;
mod matrix;
mod model;
mod vector_point;

fn put_pixel(canvas: &mut RgbImage, color: &mut Rgb<u8>, coord: Point) {
    let y_offset = CANVAS_HEIGHT / 2;
    let x_offset = CANVAS_WIDTH / 2;

    if coord.x < -x_offset || coord.x > x_offset || coord.y < -y_offset || coord.y > y_offset {
        // println!("Point - {:?} is out of bound ", coord);
        return;
    }

    canvas.put_pixel(
        (coord.x + x_offset) as u32,
        (coord.y + y_offset) as u32,
        *color,
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

        d += a;
    }

    values
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

        d += a;
    }

    values
}

fn draw_line(canvas: &mut RgbImage, point_a: &mut Point, point_b: &mut Point, color: &mut Rgb<u8>) {
    let dx = point_b.x - point_a.x;
    let dy = point_b.y - point_a.y;
    let mut p0 = point_a.clone();
    let mut p1 = point_b.clone();

    if dx.abs() > dy.abs() {
        //line is horizontalish
        if dx < 0 {
            let temp = p0;
            p0 = p1;
            p1 = temp;
        }

        let ys = interpolate(p0.x, p0.y, p1.x, p1.y);

        for x in p0.x..p1.x {
            let point = Point::new(x, ys[(x - p0.x) as usize]);
            put_pixel(canvas, color, point);
        }
    } else {
        // Line is vertical-ish
        if dy < 0 {
            let temp = p0;
            p0 = p1;
            p1 = temp;
        }

        let xs = interpolate(p0.y, p0.x, p1.y, p1.x);

        for y in p0.y..p1.y {
            let point = Point::new(xs[(y - p0.y) as usize], y);
            put_pixel(canvas, color, point);
        }
    }
}

fn draw_wireframe_triangle(
    p0: &mut Point,
    p1: &mut Point,
    p2: &mut Point,
    color: &mut Rgb<u8>,
    canvas: &mut RgbImage,
) {
    draw_line(canvas, p0, p1, color);
    draw_line(canvas, p1, p2, color);
    draw_line(canvas, p0, p2, color);
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
    let h02 = interpolate_f32(p0.y, 0., p2.y, 0.9);

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
            let mut color = color;
            color.apply(|x_in| ((x_in as f32) * h_segment[(x - x_l) as usize]).round() as u8);
            put_pixel(canvas, &mut color, Point::new(x, y))
        }
    }
}

fn project_vertex(v: HomogenousVectorPoint) -> Point {
    Point::viewport_to_canvas(
        v.values[0] * PROJECTION_PLANE_Z / v.values[2],
        v.values[1] * PROJECTION_PLANE_Z / v.values[2],
    )
}

fn render_triangle(canvas: &mut RgbImage, triangle: &mut Triangle, projected: &mut Vec<Point>) {
    // println!("{:?}", projected);
    let mut p0 = projected[triangle.vertex.0].clone();
    let mut p1 = projected[triangle.vertex.1].clone();
    let mut p2 = projected[triangle.vertex.2].clone();
    draw_wireframe_triangle(&mut p0, &mut p1, &mut p2, &mut triangle.color, canvas)
}

fn clip_triangle(
    triangle: &Triangle,
    plane: &Plane,
    triangles: &mut Vec<Triangle>,
    vertices: &Vec<VectorPoint>,
) {
    let v0 = vertices[triangle.vertex.0];
    let v1 = vertices[triangle.vertex.1];
    let v2 = vertices[triangle.vertex.2];

    let in0 = dot(
        &HomogenousVectorPoint::new(&plane.normal),
        &HomogenousVectorPoint::new(&v0),
    ) + plane.distance
        > 0.;
    let in1 = dot(
        &HomogenousVectorPoint::new(&plane.normal),
        &HomogenousVectorPoint::new(&v1),
    ) + plane.distance
        > 0.;
    let in2 = dot(
        &HomogenousVectorPoint::new(&plane.normal),
        &HomogenousVectorPoint::new(&v2),
    ) + plane.distance
        > 0.;

    let mut in_count = 0;

    if in0 {
        in_count += 1;
    }

    if in1 {
        in_count += 1;
    }

    if in2 {
        in_count += 1;
    }

    match in_count {
        0 => return,
        3 => triangles.push(triangle.clone()),
        _ => return,
    }
}

fn transform_and_clip(
    clipping_planes: &Vec<Plane>,
    model: &Model,
    scale: f32,
    transform: Matrix,
) -> Option<Model> {
    let center = transform.clone() * HomogenousVectorPoint::new(&model.bounds_center);
    println!("{:?}", center);
    let radius = model.bounds_radius * scale;

    for p in clipping_planes {
        let distance = dot(&HomogenousVectorPoint::new(&p.normal), &center) + p.distance;
        if distance < -radius {
            return None;
        }
    }

    let mut vertices: Vec<VectorPoint> = vec![];

    for v in &model.vertices {
        vertices.push(VectorPoint::from(
            transform.clone() * HomogenousVectorPoint::new(v),
        ));
    }

    let mut triangles = model.triangles.clone();

    for p in clipping_planes {
        let mut new_triangles = vec![];

        for t in &triangles {
            clip_triangle(t, p, &mut new_triangles, &vertices);
        }

        triangles = new_triangles;
    }

    return Some(Model::new(
        ModelName::CUBE,
        vertices,
        triangles,
        model.transform.clone(),
        center.into(),
        model.bounds_radius,
    ));
}

fn render_scene(canvas: &mut RgbImage, camera: Camera, instances: Vec<Model>) {
    let camera_matrix = Matrix::transpose(&camera.orientation)
        * Matrix::new_translation_matrix(-1. * camera.position);
    for i in instances {
        let transform = camera_matrix.clone() * i.transform_matrix.clone();
        let clipped = transform_and_clip(
            &camera.clipping_planes,
            &i,
            i.transform.scale,
            transform.clone(),
        );


        if clipped.is_some() {
        println!("clipped - {:?}  \n", clipped);
            render_instance(canvas, clipped.unwrap(), transform);
        }
    }
}

fn render_instance(canvas: &mut RgbImage, instance: Model, transform: Matrix) {
    let mut projected = vec![];

    for v in instance.vertices {
        let v_homogenous = HomogenousVectorPoint::new(&v);
        projected.push(project_vertex(transform.clone() * v_homogenous));
    }

    for mut t in instance.triangles {
        render_triangle(canvas, &mut t, &mut projected)
    }
}

fn main() {
    let path = Path::new("./imgs/1_draw_line.png");

    let BLUE = Rgb([0, 0, 255]);
    let RED = Rgb([255, 0, 0]);
    let GREEN = Rgb([0, 255, 0]);
    let CYAN = Rgb([0, 255, 255]);
    let PURPLE = Rgb([128, 0, 128]);
    let YELLOW = Rgb([255, 255, 0]);

    let mut canvas = RgbImage::new(
        u32::try_from(CANVAS_WIDTH + THRESHOLD_CANVAS).unwrap(),
        u32::try_from(CANVAS_HEIGHT + THRESHOLD_CANVAS).unwrap(),
    );

    for (_x, _y, pix) in canvas.enumerate_pixels_mut() {
        pix.0 = BACKGROUND_COLOR.0;
    }

    // Define vertices
    let v0 = VectorPoint::new(1., 1., 1.);
    let v1 = VectorPoint::new(-1., 1., 1.);
    let v2 = VectorPoint::new(-1., -1., 1.);
    let v3 = VectorPoint::new(1., -1., 1.);
    let v4 = VectorPoint::new(1., 1., -1.);
    let v5 = VectorPoint::new(-1., 1., -1.);
    let v6 = VectorPoint::new(-1., -1., -1.);
    let v7 = VectorPoint::new(1., -1., -1.);

    // // Define triangles
    let triangles = vec![
        Triangle::new((0, 1, 2), RED),
        Triangle::new((0, 2, 3), RED),
        Triangle::new((4, 0, 3), GREEN),
        Triangle::new((4, 3, 7), GREEN),
        Triangle::new((5, 4, 7), BLUE),
        Triangle::new((5, 7, 6), BLUE),
        Triangle::new((1, 5, 6), YELLOW),
        Triangle::new((1, 6, 2), YELLOW),
        Triangle::new((4, 5, 1), PURPLE),
        Triangle::new((4, 1, 0), PURPLE),
        Triangle::new((2, 6, 7), CYAN),
        Triangle::new((2, 7, 3), CYAN),
    ];

    let vertices = vec![v0, v1, v2, v3, v4, v5, v6, v7];

    let bounds_center = VectorPoint::new(0., 0., 0.);

    let model_instance1 = Model::new(
        ModelName::CUBE,
        vertices.clone(),
        triangles.clone(),
        Transform::new(0.75, 0, VectorPoint::new(-1.5, 0., 5.)),
        bounds_center.clone(),
        f32::sqrt(3.),
    );
    let model_instance2 = Model::new(
        ModelName::CUBE,
        vertices.clone(),
        triangles.clone(),
        Transform::new(1., 195, VectorPoint::new(1.25, 2.5, 7.5)),
        bounds_center.clone(),
        f32::sqrt(3.),
    );

    let model_instance3 = Model::new(
        ModelName::CUBE,
        vertices.clone(),
        triangles.clone(),
        Transform::new(1., 195, VectorPoint::new(0., 0., -10.)),
        bounds_center.clone(),
        f32::sqrt(3.),
    );

    let s2 = 1.0 / f32::sqrt(2.);

    let clipping_planes = vec![
        Plane::new(VectorPoint::new(0., 0., 1.), -1.), //Near
        Plane::new(VectorPoint::new(s2, 0., s2), 0.),  // Left
        Plane::new(VectorPoint::new(-s2, 0., s2), 0.), // Right
        Plane::new(VectorPoint::new(0., -s2, s2), 0.), // Top
        Plane::new(VectorPoint::new(0., s2, s2), 0.),  // Bottom
    ];

    let camera = Camera::new(
        VectorPoint::new(-3., 1., 2.),
        Matrix::new_Y_rotation_matrix(-30),
        clipping_planes,
    );
    render_scene(
        &mut canvas,
        camera,
        vec![model_instance1, model_instance2, model_instance3],
    );

    canvas.save(path).unwrap();
}
