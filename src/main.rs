#[macro_use]
extern crate impl_ops;
extern crate rayon;
extern crate sdl2;

use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub mod camera;
pub mod vector;

use crate::camera::Camera;
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
    let mut cam = Camera::new(
        Vector {
            x: 127.0,
            y: 90.0,
            z: 200.0,
        },
        25.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );
    cam.set_angle(-std::f64::consts::PI);
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
                render(&cam, &mut screen, tick);
            })
            .unwrap();
        canvas
            .copy_ex(&screen_texture, None, None, 0.0, None, false, false)
            .unwrap();
        canvas.present();

        tick += 1.0;
    }
}

fn render(cam: &Camera, screen: &mut [u8], _tick: f64) {
    screen
        .par_chunks_mut(4)
        .enumerate()
        .for_each(|(_i, pixel)| {
            pixel[0] = 255;
            pixel[1] = 0;
            pixel[2] = 255;
            pixel[3] = 255;
        });
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
