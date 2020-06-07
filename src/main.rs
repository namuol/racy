#[macro_use]
extern crate impl_ops;
extern crate rayon;
extern crate sdl2;

use core::f64::consts::PI;
use rand::prelude::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod camera;
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

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 320;

const SCREEN_SCALE: u32 = 3;

const WHITE: DiffuseColor = DiffuseColor {
    color: HDRColor {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    },
};

const RED: DiffuseColor = DiffuseColor {
    color: HDRColor {
        r: 0.92,
        g: 0.2,
        b: 0.1,
    },
};

const GREEN: DiffuseColor = DiffuseColor {
    color: HDRColor {
        r: 0.2,
        g: 0.92,
        b: 0.1,
    },
};

fn basic_scene() -> Scene {
    let mut lights: Vec<Light> = vec![];

    lights.push(Light {
        color: HDRColor {
            r: 3.0,
            g: 3.0,
            b: 3.0,
        },
        center: Vector {
            x: -3.0,
            y: 5.0,
            z: 8.0,
        },
        radius: 0.0,
    });

    Scene {
        bg_color: HDRColor {
            // r: (98.0 / 255.0),
            // g: (192.0 / 255.0),
            // b: (255.0 / 255.0),
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        lights,
        photons: vec![],
        cam: Camera::new(
            Vector {
                x: 0.0,
                y: 0.0, // meters
                z: 0.0,
            },
            45.0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        ),
        renderables: vec![
            Box::new(Sphere::new(
                Vector {
                    x: -2.0,
                    y: 1.0,
                    z: 12.0,
                },
                1.0,
                &WHITE,
            )),
            Box::new(Sphere::new(
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 8.0,
                },
                1.0,
                &MIRROR,
            )),
            Box::new(Sphere::new(
                Vector {
                    x: 2.0,
                    y: 1.0,
                    z: 8.0,
                },
                1.0,
                &WHITE,
            )),
            // "Floor"
            Box::new(Plane::new(
                Vector {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                Vector {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                &WHITE,
            )),
            // "Back wall"
            Box::new(Plane::new(
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 14.0,
                },
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                &MIRROR,
            )),
            // "Left wall"
            Box::new(Plane::new(
                Vector {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
                &RED,
            )),
            // "Right wall"
            Box::new(Plane::new(
                Vector {
                    x: -4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                &GREEN,
            )),
            // "Front wall"
            Box::new(Plane::new(
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                &WHITE,
            )),
            // // "Ceiling"
            Box::new(Plane::new(
                Vector {
                    x: 0.0,
                    y: 8.0,
                    z: 0.0,
                },
                Vector {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                &WHITE,
            )),
        ],
    }
}

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
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut tick: f64 = 0.0;

    let mut scene = basic_scene();

    // scene.lights.clear(); // Turn off all lights

    canvas.set_draw_color::<HDRColor>(scene.bg_color.into());
    canvas.clear();
    canvas.present();
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
        let mut photons = vec![
            Light {
                color: HDRColor {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0
                },
                center: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                radius: 0.0,
            };
            1000
        ];

        // Generate point light sources by shooting lots of rays into the scene from
        // our light sources.
        photons.par_chunks_mut(1).for_each(|photon| {
            let mut rng = thread_rng();
            match scene.lights.choose(&mut rng) {
                None => (),
                Some(light) => {
                    let ray = Ray {
                        origin: light.center,
                        direction: Vector::random_norm(),
                    };
                    match scene.cast(&ray, 0) {
                        None => (),
                        Some(intersection) => {
                            let point = ray.origin + ray.direction * intersection.t;
                            let object = &scene.renderables[intersection.renderable_idx];
                            let normal = object.normal(&point);
                            let color = object
                                .material()
                                .color_at(&mut rng, &point, &normal, &ray, &scene, 0);
                            photon[0].center = point + (normal * 0.001);
                            photon[0].color = color;
                        }
                    }
                }
            }
        });

        // let total_photon_power = photons.par_iter().fold(
        //     || HDRColor {
        //         r: 0.0,
        //         g: 0.0,
        //         b: 0.0,
        //     },
        //     |acc, photon| acc + photon.color,
        // );

        scene.photons = photons;

        screen_texture
            .with_lock(None, |mut screen, _size| {
                render(&scene, &mut screen);
            })
            .unwrap();
        canvas
            .copy_ex(&screen_texture, None, None, 0.0, None, false, false)
            .unwrap();
        canvas.present();
        // scene.cam.set_angle(PI + PI / 20.0 * (tick * 0.045).sin());
        // scene.cam.eye.x = 3.2 * (tick * 0.03).sin();
        // scene.cam.eye.z = -2.0 + 1.0 * (tick * 0.03).cos();
        // scene.cam.eye.y = 0.2 + 1.0 * (tick * 0.01).sin();
        scene.lights[0].center.x = 3.2 * (tick * 0.03).sin();
        scene.lights[0].center.z = 7.0 + 3.2 * (tick * 0.03).cos();
        scene.lights[0].center.y = 3.2 + 2.0 * (tick * 0.02).cos();
        tick += 1.0;
    }
}

const EXPOSURE: f32 = 1.0;
const GAMMA: f32 = 1.0;

fn render(scene: &Scene, screen: &mut [u8]) {
    let cam = scene.cam;
    let screen_width = cam.screen_width as usize;
    screen.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
        let x = i % screen_width;
        let y = i / screen_width;

        let pixel_ray = cam.get_ray_from_uv(x as f32, y as f32);

        let mut rng = thread_rng();

        match scene.cast(&pixel_ray, 0) {
            None => (),
            Some(intersection) => {
                let point = pixel_ray.origin + pixel_ray.direction * intersection.t;
                let object = &scene.renderables[intersection.renderable_idx];
                let normal = object.normal(&point);
                let color = object
                    .material()
                    .color_at(&mut rng, &point, &normal, &pixel_ray, &scene, 0);
                let display_rgb = color.into_display_rgb(EXPOSURE, GAMMA);
                pixel[0] = display_rgb.b;
                pixel[1] = display_rgb.g;
                pixel[2] = display_rgb.r;
                pixel[3] = display_rgb.a;
            }
        }
    });
}
