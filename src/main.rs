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
pub mod plane;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod vector;

use crate::camera::*;
use crate::material::*;
use crate::plane::*;
use crate::ray::*;
use crate::scene::*;
use crate::sphere::*;
use crate::vector::*;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 800;

const SCREEN_SCALE: u32 = 1;

const FOG_COLOR: RGB = RGB {
    r: 98,
    g: 192,
    b: 255,
};

const WHITE: DiffuseColor = DiffuseColor {
    color: HDRColor {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    },
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
                    x: -1.1,
                    y: 0.0,
                    z: 6.0,
                },
                1.0,
                &WHITE,
            )),
            Box::new(Sphere::new(
                Vector {
                    x: 1.1,
                    y: 0.0,
                    z: 6.0,
                },
                1.0,
                &WHITE,
            )),
            Box::new(Plane::new(
                Vector {
                    x: 0.0,
                    y: -2.0,
                    z: 0.0,
                },
                Vector {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                &WHITE,
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
        scene.cam.set_angle(PI + PI / 6.0 * (tick * 0.03).sin());
        tick += 1.0;
    }
}

fn render(scene: &Scene, screen: &mut [u8]) {
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

fn cast(ray: &Ray, scene: &Scene) -> Option<HDRColor> {
    let mut intersection_obj: Option<(Vector, &dyn Material, Vector, f64)> = None;
    for object in &scene.renderables {
        match object.intersects(ray) {
            None => continue,
            Some(this_intersection) => match intersection_obj {
                None => {
                    let this_dist_squared = (ray.origin - this_intersection).length_squared();
                    intersection_obj = Some((
                        this_intersection,
                        object.material(),
                        object.normal(&this_intersection),
                        this_dist_squared,
                    ));
                }
                Some((_, _, _, closest_dist_squared)) => {
                    let this_dist_squared = (ray.origin - this_intersection).length_squared();
                    if this_dist_squared < closest_dist_squared {
                        intersection_obj = Some((
                            this_intersection,
                            object.material(),
                            object.normal(&this_intersection),
                            this_dist_squared,
                        ));
                    }
                }
            },
        }
    }

    if let Some((intersection, material, normal, _)) = intersection_obj {
        return Some(material.color_at(&intersection, &normal, &scene));
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
