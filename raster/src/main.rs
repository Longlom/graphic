use image::{Pixel, Rgb, RgbImage};
use raster::*;
use std::{path::Path};

fn put_pixel(canvas: &mut RgbImage, color: &mut Rgb<u8>, coord: Point) {
    let y_offset = CANVAS_HEIGHT / 2;
    let x_offset = CANVAS_WIDTH / 2;

    if coord.x < -x_offset || coord.x > x_offset || coord.y < -y_offset || coord.y > y_offset {
        println!("Point - {:?} is out of bound ", coord);
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

fn draw_line(
    canvas: &mut RgbImage,
    point_a: &mut Point,
    point_b: &mut Point,
    color: &mut Rgb<u8>,
) {
    let dx = point_b.x - point_a.x;
    let dy = point_b.y - point_a.y;

    if dx.abs() > dy.abs() {
        //line is horizontalish
        if point_a.x > point_b.x {
            Point::swap(point_a, point_b);
        }

        let ys = interpolate(point_a.x, point_a.y, point_b.x, point_b.y);

        for x in point_a.x..point_b.x {
            let point = Point::new(x, ys[(x - point_a.x) as usize]);
            put_pixel(canvas, color, point);
        }
    } else {
        // Line is vertical-ish
        if point_a.y > point_b.y {
            Point::swap(point_a, point_b);
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
    color: &mut Rgb<u8>,
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

fn project_vertex(v: VectorPoint) -> Point {
    Point::viewport_to_canvas(
        v.x * PROJECTION_PLANE_Z / v.z,
        v.y * PROJECTION_PLANE_Z / v.z,
    )
}

fn render_triangle(canvas: &mut RgbImage, triangle: &mut Triangle, projected: &mut Vec<Point>) {
    let mut p0 = projected[triangle.vertex.0].clone();
    let mut p1 = projected[triangle.vertex.1].clone();
    let mut p2 = projected[triangle.vertex.2].clone();
    draw_wireframe_triangle(&mut p0, &mut p1, &mut p2, &mut triangle.color, canvas)
}

fn render_object(canvas: &mut RgbImage, vertices: Vec<VectorPoint>, triangles: Vec<Triangle>) {
    let mut projected = vec![];

    for v in vertices {
        projected.push(project_vertex(v));
    }

    for mut t in triangles {
        render_triangle(canvas, &mut t, &mut projected);
    }
}

fn render_scene(canvas: &mut RgbImage, instances: Vec<Model>) {
    for i in instances {
        render_instance(canvas, i);
    }
}

fn render_instance(canvas: &mut RgbImage, instance: Model) {
    let mut projected = vec![];

    for v in instance.vertices {
        let v_transformed = apply_transform(v, instance.transform.clone());
        projected.push(project_vertex(v_transformed));
    }

    for mut t in instance.triangles {
        render_triangle(canvas, &mut t, &mut projected)
    }
}

fn apply_transform(v: VectorPoint, transform: Transform) -> VectorPoint {
    let res = v.scale(transform.scale);
    let res = res.rotate(transform.rotation);
    
    res.translate(transform.translation)
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

    // draw_filled_triangle(
    //     &mut Point { x: 0, y: 0 },
    //     &mut Point { x: -40, y: 300 },
    //     &mut Point { x: 300, y: 100 },
    //     Rgb([0, 255, 0]),
    //     &mut canvas,
    // );

    // The four "front" vertices
    // let va_f = VectorPoint::new(-1., 1., 1.);
    // let vb_f = VectorPoint::new(1., 1., 1.);
    // let vc_f = VectorPoint::new(1., -1., 1.);
    // let vd_f = VectorPoint::new(-1., -1., 1.);

    // // The four "back" vertices
    // let va_b = VectorPoint::new(-1., 1., 2.);
    // let vb_b = VectorPoint::new(1., 1., 2.);
    // let vc_b = VectorPoint::new(1., -1., 2.);
    // let vd_b = VectorPoint::new(-1., -1., 2.);

    // let t = VectorPoint::new(-1.5, 0., 3.);

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

    let model_instance1 = Model::new(
        ModelName::CUBE,
        vertices.clone(),
        triangles.clone(),
        Transform::new(1.5, 45, VectorPoint::new(-1.5, 0., 7.)),
    );
    // let model_instance2 = Model::new(
    //     ModelName::CUBE,
    //     vertices.clone(),
    //     triangles.clone(),
    //     Transform::new(1.5, 45, VectorPoint::new(1.5, 1., 6.)),
    // );

    render_scene(&mut canvas, vec![model_instance1, 
        // model_instance2
        ]);

    // render_object(&mut canvas, vec![v0, v1, v2, v3, v4, v5, v6, v7], triangles);
    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(va_f),
    //     &mut project_vertex(vb_f),
    //     &mut BLUE,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vb_f),
    //     &mut project_vertex(vc_f),
    //     &mut BLUE,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vc_f),
    //     &mut project_vertex(vd_f),
    //     &mut BLUE,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vd_f),
    //     &mut project_vertex(va_f),
    //     &mut BLUE,
    // );

    // // BackFace
    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(va_b),
    //     &mut project_vertex(vb_b),
    //     &mut RED,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vb_b),
    //     &mut project_vertex(vc_b),
    //     &mut RED,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vc_b),
    //     &mut project_vertex(vd_b),
    //     &mut RED,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vd_b),
    //     &mut project_vertex(va_b),
    //     &mut RED,
    // );

    // // The front-to-back edges
    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(va_f),
    //     &mut project_vertex(va_b),
    //     &mut GREEN,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vb_f),
    //     &mut project_vertex(vb_b),
    //     &mut GREEN,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vc_f),
    //     &mut project_vertex(vc_b),
    //     &mut GREEN,
    // );

    // draw_line(
    //     &mut canvas,
    //     &mut project_vertex(vd_f),
    //     &mut project_vertex(vd_b),
    //     &mut GREEN,
    // );

    canvas.save(path).unwrap();
}
