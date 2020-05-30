#[macro_use]
extern crate impl_ops;
extern crate rayon;
extern crate sdl2;

use core::f64::consts::PI;
use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub mod camera;
pub mod line;
pub mod ray;
pub mod vector;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::vector::Vector;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 400;

const SCREEN_SCALE: u32 = 2;

const FOG_COLOR: RGB = RGB {
    r: 98,
    g: 192,
    b: 255,
};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "racy",
            SCREEN_WIDTH * SCREEN_SCALE,
            SCREEN_HEIGHT * SCREEN_SCALE,
        )
        .position_centered()
        .build()
        .unwrap();

    let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG);
    let cam = Camera::new(
        Vector {
            x: 0.0,
            y: 2.0, // meters
            z: 0.0,
        },
        25.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut screen_texture = texture_creator
        .create_texture_streaming(
            texture_creator.default_pixel_format(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .unwrap();
    screen_texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(FOG_COLOR);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut tick: f64 = 0.0;
    let mut objects: Vec<Sphere> = vec![
        Sphere {
            color: RGB { r: 255, g: 0, b: 0 },
            center: Vector {
                x: -2.0,
                y: 2.0,
                z: 12.0,
            },
            radius: 1.0,
        },
        Sphere {
            color: RGB { r: 0, g: 255, b: 0 },
            center: Vector {
                x: 0.0,
                y: 2.0,
                z: 12.0,
            },
            radius: 1.0,
        },
        Sphere {
            color: RGB { r: 0, g: 0, b: 255 },
            center: Vector {
                x: 2.0,
                y: 2.0,
                z: 12.0,
            },
            radius: 1.0,
        },
    ];
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        screen_texture
            .with_lock(None, |mut screen, _size| {
                render(&cam, &mut screen, &objects);
            })
            .unwrap();
        canvas
            .copy_ex(&screen_texture, None, None, 0.0, None, false, false)
            .unwrap();
        canvas.present();
        let mut i = 0.0;
        for mut sphere in &mut objects {
            sphere.center.y = 2.0 + 0.5 * ((tick) * 0.01 + i * PI * 0.45).sin();
            i += 1.0;
        }
        tick += 1.0;
    }
}

fn render<S, I>(cam: &Camera, screen: &mut [u8], objects: &Vec<S>)
where
    I: Intersection,
    S: IntersectsWithRay<I> + Sync + Colorful,
{
    let screen_width = cam.screen_width as usize;
    screen.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
        let x = i % screen_width;
        let y = i / screen_width;

        let pixel_ray = cam.get_ray_from_uv(x, y);

        match cast(&pixel_ray, &objects) {
            None => (),
            Some(color) => {
                pixel[0] = color.r;
                pixel[1] = color.g;
                pixel[2] = color.b;
                pixel[3] = 255;
            }
        }
    });
}

fn cast<S, I>(ray: &Ray, objects: &Vec<S>) -> Option<RGB>
where
    I: Intersection,
    S: IntersectsWithRay<I> + Colorful,
{
    for object in objects {
        match object.intersects(ray) {
            // For now we just return the first we intersect with:
            Some(_) => return Some(object.color()),
            None => continue,
        }
    }
    None
}

trait Colorful {
    fn color(&self) -> RGB;
}

trait Intersection {}
trait IntersectsWithRay<I>
where
    I: Intersection,
{
    fn intersects(&self, ray: &Ray) -> Option<I>
    where
        I: Intersection;
}

struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub color: RGB,
}

struct SphereIntersection {}
impl Intersection for SphereIntersection {}

impl Colorful for Sphere {
    fn color(&self) -> RGB {
        self.color
    }
}

impl IntersectsWithRay<SphereIntersection> for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<SphereIntersection> {
        let ray_origin_to_sphere_center = self.center - ray.origin;
        let dist_to_sphere_center_squared = ray_origin_to_sphere_center.length_squared();
        let dist_to_sphere_perpendicular_squared =
            ray_origin_to_sphere_center.dot(&ray.direction).powi(2);

        if (dist_to_sphere_center_squared - dist_to_sphere_perpendicular_squared)
            < self.radius.powi(2)
        {
            return Some(SphereIntersection {});
        }
        None
    }
}

#[derive(Clone, Copy)]
struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<Color> for RGB {
    fn into(self) -> Color {
        Color::RGB(self.r, self.g, self.b)
    }
}
