use crate::intersection::*;
use crate::material::HDRColor;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Plane {
  pub center: Vector,
  normal: Vector,
}

impl Plane {
  pub fn new(center: Vector, normal: Vector) -> Self {
    Plane {
      center,
      normal: normal.normalized(),
    }
  }
}

impl IntersectsWithRay<PlaneIntersection> for Plane {
  fn intersects(&self, ray: &Ray) -> Option<PlaneIntersection> {
    let denominator = self.normal.dot(&ray.direction);
    if denominator.abs() < 0.0001 {
      return None;
    }
    let t = (self.center - ray.origin).dot(&self.normal) / denominator;

    if t < 0.0001 {
      return None;
    }

    Some(PlaneIntersection {
      t,
      normal: self.normal,
      ray: *ray,
    })
  }
}

pub struct PlaneIntersection {
  t: f64,
  normal: Vector,
  ray: Ray,
}

impl Intersection for PlaneIntersection {
  fn point(&self) -> Vector {
    self.ray.direction * self.t
  }

  fn normal(&self) -> Vector {
    self.normal
  }

  fn dist_squared(&self) -> f64 {
    (self.point() - self.ray.origin).length_squared()
  }
}

// TODO: Refactor things so it's easier to generalize materials independently of
// objects
const POINT_LIGHT: Vector = Vector {
  x: 0.0,
  y: 4.0,
  z: 14.0,
};

const POINT_LIGHT_POWER: f64 = 4.0;

impl Material for Plane {
  fn color_at<I>(&self, intersection: &I) -> HDRColor
  where
    I: Intersection,
  {
    // For debugging normals:
    //
    // let normal = intersection.normal();
    // return HDRColor {
    //   r: ((1.0 + normal.x) / 2.0) as f32,
    //   g: ((1.0 + normal.y) / 2.0) as f32,
    //   b: (0.5 - normal.z) as f32,
    // };

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
    let to_light = POINT_LIGHT - intersection.point();

    // 2. Use the dot product to calculate theta.cos()
    let theta_cos = to_light.dot(&intersection.normal());

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
