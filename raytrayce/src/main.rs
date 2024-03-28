use image::{Rgb, RgbImage};
use std::path::Path;

struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Sphere {
    center: VectorPoint,
    radius: f32,
    color: Rgb<u8>,
    specular: f32,
    reflective: f32,
}

const SPHERE_1: Sphere = Sphere {
    center: (0., -1., 3.),
    radius: 1.0,
    color: Rgb([255u8, 0u8, 0u8]),
    specular: 500.,
    reflective: 0.2,
};

const SPHERE_2: Sphere = Sphere {
    center: (2., 0., 4.),
    radius: 1.0,
    color: Rgb([0u8, 0u8, 255u8]),
    specular: 500.,
    reflective: 0.3,
};

const SPHERE_3: Sphere = Sphere {
    center: (-2., 0., 4.),
    radius: 1.0,
    color: Rgb([0u8, 255u8, 0u8]),
    specular: 10.,
    reflective: 0.4,
};

const SPHERE_4: Sphere = Sphere {
    center: (0., -5001., 0.),
    radius: 5000.0,
    color: Rgb([255u8, 255u8, 0u8]),
    specular: 1000.,
    reflective: 0.5,
};

type VectorPoint = (f32, f32, f32);

const CANVAS_WIDTH: i32 = 1500;
const CANVAS_HEIGHT: i32 = 1500;
const VIEWPORT_SIZE: f32 = 2.0;
const PROJECTION_PLANE_Z: f32 = 1.0;
const BACKGROUND_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

#[derive(PartialEq)]
enum LightType {
    Ambient,
    Point,
    Directional,
}

struct Light {
    intensity: f32,
    light_type: LightType,
    direction: Option<VectorPoint>,
}

const LIGHT_1: Light = Light {
    light_type: LightType::Ambient,
    intensity: 0.2,
    direction: None,
};

const LIGHT_2: Light = Light {
    light_type: LightType::Point,
    intensity: 0.6,
    direction: Some((2., 1., 0.)),
};

const LIGHT_3: Light = Light {
    light_type: LightType::Directional,
    intensity: 0.2,
    direction: Some((1., 4., 4.)),
};

const CAMERA_ROTATION: [[f32; 3]; 3] = [[0.7071, 0., -0.7071], [0., 1., 0.], [0.7071, 0., 0.7071]];

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

fn intersect_ray_sphere(
    origin: VectorPoint,
    direction: VectorPoint,
    sphere: &Sphere,
) -> (f32, f32) {
    let r = sphere.radius;
    let c0 = substract_vector(origin, sphere.center);

    let a = dot_vector(direction, direction);
    let b = 2. * dot_vector(c0, direction);
    let c = dot_vector(c0, c0) - r * r;

    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return (f32::INFINITY, f32::INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2. * a);
    let t2 = (-b - discriminant.sqrt()) / (2. * a);
    (t1, t2)
}

fn closest_intersection(
    origin: VectorPoint,
    direction: VectorPoint,
    t_min: f32,
    t_max: f32,
) -> (Option<&'static Sphere>, f32) {
    let mut closest_t = f32::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;

    for sphere in [&SPHERE_1, &SPHERE_2, &SPHERE_3, &SPHERE_4] {
        let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);
        if t1 > t_min && t1 < t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if t2 > t_min && t2 < t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    (closest_sphere, closest_t)
}

fn reflect_ray(r_vector: VectorPoint, normal: VectorPoint) -> VectorPoint {
    substract_vector(
        dot_number(dot_number(normal, dot_vector(normal, r_vector)), 2.),
        r_vector,
    )
}

fn trace_ray(
    origin: VectorPoint,
    direction: VectorPoint,
    t_min: f32,
    t_max: f32,
    rec_depth: u32,
) -> Rgb<u8> {
    let (closest_sphere, closest_t) = closest_intersection(origin, direction, t_min, t_max);

    match closest_sphere {
        Some(sphere) => {
            let position = add_vector(origin, dot_number(direction, closest_t));
            let normal = substract_vector(position, closest_sphere.unwrap().center);
            let normal = divide_number(normal, length(normal));
            let lightning_koef = compute_lightning(
                position,
                normal,
                negate(direction),
                sphere.specular.round() as i32,
            );
            let mut local_color = sphere
                .color
                .0
                .map(|color| (f32::try_from(color).unwrap() * lightning_koef))
                .map(|color| color.round() as u8);

            let reflective = sphere.reflective;

            if rec_depth <= 0 || reflective <= 0. {
                return Rgb(local_color);
            }

            let reflect_ray = reflect_ray(negate(direction), normal);
            let reflected_color =
                trace_ray(position, reflect_ray, 0.001, f32::INFINITY, rec_depth - 1);
            // let mut result_color: [u8; 3] = [0; 3];

            for (index, col) in local_color.iter_mut().enumerate() {
                *col = (f32::try_from(*col).unwrap() * (1. - reflective)
                    + f32::try_from(reflected_color.0[index]).unwrap() * reflective)
                    .round() as u8;
            }
            Rgb(local_color)
        }
        None => {
            BACKGROUND_COLOR
        }
    }
}

fn compute_lightning(
    position: VectorPoint,
    normal: VectorPoint,
    vector: VectorPoint,
    specular: i32,
) -> f32 {
    let mut i = 0.;
    for light in [&LIGHT_1, &LIGHT_2, &LIGHT_3] {
        match light.light_type {
            LightType::Ambient => {
                i += light.intensity;
            }
            _ => {
                let light_direction: VectorPoint;
                let t_max: f32;
                if light.light_type == LightType::Point {
                    t_max = 1.;
                    light_direction = substract_vector(light.direction.unwrap(), position);
                } else {
                    light_direction = light.direction.unwrap();
                    t_max = f32::INFINITY;
                }

                let (shadow_sphere, _shadow_t) =
                    closest_intersection(position, light_direction, 0.001, t_max);
                if shadow_sphere.is_some() {
                    continue;
                }

                let normal_dot_1 = dot_vector(normal, light_direction);
                if normal_dot_1 > 0. {
                    i +=
                        light.intensity * normal_dot_1 / (length(normal) * length(light_direction));
                }

                if specular != -1 {
                    let reflection = substract_vector(
                        dot_number(dot_number(normal, 2.), dot_vector(normal, light_direction)),
                        light_direction,
                    );
                    let reflection_dot_v = dot_vector(reflection, vector);
                    if reflection_dot_v > 0. {
                        i += light.intensity
                            * f32::powf(
                                reflection_dot_v / (length(reflection) * length(vector)),
                                specular as f32,
                            )
                    }
                }
            }
        }
    }

    i
}


fn rotate_camera(direction: VectorPoint) -> VectorPoint {
    let mut result = [0., 0., 0.];
    let array_direction = [direction.0, direction.1, direction.2];

    for i in 0..3 {
        for j in 0..3 {
            result[i] += array_direction[j] * CAMERA_ROTATION[i][j]
        }
    }

    (result[0], result[1], result[2])
}
fn main() {
    let path = Path::new("./imgs/5_rotation.png");

    let mut canvas = RgbImage::new(
        u32::try_from(CANVAS_WIDTH).unwrap(),
        u32::try_from(CANVAS_HEIGHT).unwrap(),
    );
    let camera_position: VectorPoint = (0., 0., 0.);
    // let pool = rayon::ThreadPoolBuilder::new().num_threads(20).build().unwrap();

    for x in -CANVAS_WIDTH / 2..CANVAS_WIDTH / 2 {
        for y in -CANVAS_HEIGHT / 2..CANVAS_HEIGHT / 2 {
            // pool.install(|| {
                let direction = canvas_to_viewport(x as f32, y as f32);
                // let direction = rotate_camera(direction);
                let color = trace_ray(camera_position, direction, 1., f32::INFINITY, 3);
                put_pixel(
                    &mut canvas,
                    color,
                    Point {
                        x: x + CANVAS_WIDTH / 2,
                        y: CANVAS_HEIGHT / 2 - y,
                    },
                )
            // });
        
        }
    }

    canvas.save(path).unwrap();
}
