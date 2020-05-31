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
pub mod intersection;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vector;

use crate::camera::Camera;
use crate::intersection::*;
use crate::material::{HDRColor, Material};
use crate::ray::Ray;
use crate::sphere::Sphere;
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
    let mut scene = Scene {
        cam: Camera::new(
            Vector {
                x: 0.0,
                y: 0.0, // meters
                z: 0.0,
            },
            25.0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        ),
        renderables: vec![
            Box::new(Sphere::new(
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                0.5,
            )),
            Box::new(Sphere::new(
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 3.0,
                },
                0.25,
            )),
        ],
    };
    scene.cam.set_angle(PI);

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
                render(&scene, &mut screen);
            })
            .unwrap();
        canvas
            .copy_ex(&screen_texture, None, None, 0.0, None, false, false)
            .unwrap();
        canvas.present();

        scene.renderables[1].center.x = 0.5 * (tick * 0.01).sin();
        scene.renderables[1].center.z = 4.0 + 0.5 * (tick * 0.01).cos();

        tick += 1.0;
    }
}

struct Scene<R> {
    cam: Camera,
    renderables: Vec<Box<R>>,
}

fn render<I, R>(scene: &Scene<R>, screen: &mut [u8])
where
    I: Intersection,
    R: Material + IntersectsWithRay<I> + Sync,
{
    let cam = scene.cam;
    let screen_width = cam.screen_width as usize;
    screen.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
        let x = i % screen_width;
        let y = i / screen_width;

        let pixel_ray = cam.get_ray_from_uv(x, y);

        match cast(&pixel_ray, &scene) {
            None => (),
            Some(color) => {
                let color: RGB = color.into();
                pixel[0] = color.b;
                pixel[1] = color.g;
                pixel[2] = color.r;
                pixel[3] = 255;
            }
        }
    });
}

fn cast<I, R>(ray: &Ray, scene: &Scene<R>) -> Option<HDRColor>
where
    I: Intersection,
    R: Material + IntersectsWithRay<I>,
{
    let mut intersection_obj: Option<(I, &R, f64)> = None;
    for object in &scene.renderables {
        match object.intersects(ray) {
            None => continue,
            Some(this_intersection) => match intersection_obj {
                None => {
                    let this_dist_squared = this_intersection.dist_squared();
                    intersection_obj = Some((this_intersection, &object, this_dist_squared));
                }
                Some((_, _, closest_dist_squared)) => {
                    let this_dist_squared = this_intersection.dist_squared();
                    if this_dist_squared < closest_dist_squared {
                        intersection_obj = Some((this_intersection, &object, this_dist_squared));
                    }
                }
            },
        }
    }

    if let Some((intersection, object, _)) = intersection_obj {
        return Some(object.color_at(&intersection));
    }

    None
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

impl Into<RGB> for HDRColor {
    fn into(self) -> RGB {
        RGB {
            r: (self.r * 255.0).floor().min(255.0).max(0.0) as u8,
            g: (self.g * 255.0).floor().min(255.0).max(0.0) as u8,
            b: (self.b * 255.0).floor().min(255.0).max(0.0) as u8,
        }
    }
}
