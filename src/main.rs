#[macro_use]
extern crate impl_ops;
extern crate rayon;
extern crate sdl2;
use crate::sphere::SphereIntersection;
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
    let mut renderables = vec![
        Box::new(Sphere::new(
            Vector {
                x: -2.0,
                y: 2.0,
                z: 12.0,
            },
            1.0,
        )),
        Box::new(Sphere::new(
            Vector {
                x: 0.0,
                y: 2.0,
                z: 12.0,
            },
            1.0,
        )),
        Box::new(Sphere::new(
            Vector {
                x: 2.0,
                y: 2.0,
                z: 12.0,
            },
            1.0,
        )),
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
                render(&cam, &mut screen, &renderables);
            })
            .unwrap();
        canvas
            .copy_ex(&screen_texture, None, None, 0.0, None, false, false)
            .unwrap();
        canvas.present();
        let mut i = 0.0;
        for mut sphere in &mut renderables {
            sphere.center.y = 2.0 + 0.5 * ((tick) * 0.01 + i * PI * 0.45).sin();
            i += 1.0;
        }
        tick += 1.0;
    }
}

fn render<I, R>(cam: &Camera, screen: &mut [u8], renderables: &Vec<Box<R>>)
where
    I: Intersection,
    R: Material + IntersectsWithRay<I> + Sync,
{
    let screen_width = cam.screen_width as usize;
    screen.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
        let x = i % screen_width;
        let y = i / screen_width;

        let pixel_ray = cam.get_ray_from_uv(x, y);

        match cast(&pixel_ray, &renderables) {
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

const DEFAULT_SPHERE_COLOR: HDRColor = HDRColor {
    r: 1.0,
    g: 1.0,
    b: 0.0,
};

impl Material for Sphere {
    fn color_at<I>(&self, _: &I) -> HDRColor
    where
        I: intersection::Intersection,
    {
        DEFAULT_SPHERE_COLOR
    }
}

fn cast<'a, I, R>(ray: &Ray, renderables: &'a Vec<Box<R>>) -> Option<RGB>
where
    I: Intersection,
    R: Material + IntersectsWithRay<I>,
{
    for object in renderables {
        match object.intersects(ray) {
            // For now we just return the first we intersect with:
            Some(intersection) => return Some(object.color_at(&intersection).into()),
            None => continue,
        }
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
            r: (self.r * 255.0).floor().min(255.0) as u8,
            g: (self.g * 255.0).floor().min(255.0) as u8,
            b: (self.b * 255.0).floor().min(255.0) as u8,
        }
    }
}
