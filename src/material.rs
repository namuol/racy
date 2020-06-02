use crate::scene::Scene;
use crate::vector::Vector;

pub trait Material: Sync {
  fn color_at(&self, point: &Vector, normal: &Vector, scene: &Scene) -> HDRColor;
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

const POINT_LIGHT: Vector = Vector {
  x: 1.0,
  y: 5.0,
  z: 2.0,
};

const POINT_LIGHT_POWER: f64 = 4.0;

impl Material for DiffuseColor {
  fn color_at(&self, point: &Vector, normal: &Vector, _: &Scene) -> HDRColor {
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
    let to_light = POINT_LIGHT - point;

    // 2. Use the dot product to calculate theta.cos()
    let theta_cos = to_light.dot(&normal);

    // 3. We employ the inverse-square law to determine how intense the light
    //    should be:
    let intensity = POINT_LIGHT_POWER / (to_light.length_squared());

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

pub struct DebugNormals;

impl Material for DebugNormals {
  fn color_at(&self, _: &Vector, normal: &Vector, _: &Scene) -> HDRColor {
    return HDRColor {
      r: ((1.0 + normal.x) / 2.0) as f32,
      g: ((1.0 + normal.y) / 2.0) as f32,
      b: (0.5 - normal.z) as f32,
    };
  }
}

pub const DEBUG_NORMALS: DebugNormals = DebugNormals {};
