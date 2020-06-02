use sdl2::pixels::Color;
use std::ops;

use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub trait Material: Sync {
  fn color_at(
    &self,
    point: &Vector,
    normal: &Vector,
    ray: &Ray,
    scene: &Scene,
    depth: u8,
  ) -> HDRColor;
}

#[derive(Copy, Clone)]
pub struct HDRColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

pub struct DiffuseColor {
  pub color: HDRColor,
}

impl_op_ex!(*|a: &HDRColor, b: f32| -> HDRColor {
  HDRColor {
    r: a.r * b,
    g: a.g * b,
    b: a.b * b,
  }
});

const BLACK: HDRColor = HDRColor {
  r: 0.0,
  g: 0.0,
  b: 0.0,
};

impl Material for DiffuseColor {
  fn color_at(
    &self,
    point: &Vector,
    normal: &Vector,
    _: &Ray,
    scene: &Scene,
    depth: u8,
  ) -> HDRColor {
    if depth > MAX_MIRROR_DEPTH {
      return BLACK;
    }

    // ```text
    //                * <-light.origin
    //                |
    //                |
    //  normal        |
    //  ""..__  theta |     , - ~ ~ ~ - ,
    //        ""..__  | , '               ' ,
    //  *-----------""*                       ,
    //  ^ray.origin   ^point                   '
    //              ,                           ,
    //              ,                           ,
    // ```
    //
    // Let's implement diffuse ("Lambertian") reflection.
    //
    // To do this, all we need is the angle between the light source and our
    // normal.

    // 1. Draw a vector from our intersection point to the light source:
    let to_light = scene.light_pos - point;
    match scene.cast(
      &Ray {
        origin: *point,
        direction: to_light.normalized(),
      },
      depth + 1,
    ) {
      Some(_) => BLACK,
      None => {
        // 2. Use the dot product to calculate theta.cos()
        let theta_cos = to_light.dot(&normal);

        // 3. We employ the inverse-square law to determine how intense the light
        //    should be:
        let intensity = scene.light_power / (to_light.length_squared());

        // 4. Finally, we just multiply our lighting intensity by the cosine of the
        //    angle between our normal and the incoming light:
        let illumination = intensity * theta_cos;

        HDRColor {
          r: illumination as f32,
          g: illumination as f32,
          b: illumination as f32,
        }
      }
    }
  }
}

pub struct DebugNormals;

impl Material for DebugNormals {
  fn color_at(&self, _: &Vector, normal: &Vector, _: &Ray, _: &Scene, _depth: u8) -> HDRColor {
    return HDRColor {
      r: ((1.0 + normal.x) / 2.0) as f32,
      g: ((1.0 + normal.y) / 2.0) as f32,
      b: (0.5 - normal.z) as f32,
    };
  }
}

pub const DEBUG_NORMALS: DebugNormals = DebugNormals {};

pub struct Mirror {}

const MAX_MIRROR_DEPTH: u8 = 4;
impl Material for Mirror {
  fn color_at(
    &self,
    point: &Vector,
    normal: &Vector,
    ray: &Ray,
    scene: &Scene,
    depth: u8,
  ) -> HDRColor {
    if depth > MAX_MIRROR_DEPTH {
      return BLACK;
    }
    let neg_norm = normal * -1.0;
    let mirror_direction = ray.direction - neg_norm * 2.0 * (ray.direction.dot(&neg_norm));
    let ray_reflection = Ray {
      origin: *point,
      direction: mirror_direction,
    };
    (match scene.cast(&ray_reflection, depth + 1) {
      Some(color) => color,
      None => scene.bg_color,
    }) * 0.8
  }
}

impl Into<Color> for HDRColor {
  fn into(self) -> Color {
    Color::RGB(
      (self.r * 255.0).floor().min(255.0).max(0.0) as u8,
      (self.g * 255.0).floor().min(255.0).max(0.0) as u8,
      (self.b * 255.0).floor().min(255.0).max(0.0) as u8,
    )
  }
}

pub const MIRROR: Mirror = Mirror {};
