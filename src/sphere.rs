use crate::intersection::*;
use crate::material::HDRColor;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
  pub center: Vector,
  pub radius: f64,
  pub radius_squared: f64,
}

impl Sphere {
  pub fn new(center: Vector, radius: f64) -> Self {
    Sphere {
      center,
      radius,
      radius_squared: radius * radius,
    }
  }
}

pub struct SphereIntersection {
  radius_squared: f64,
  ray_direction: Vector,
  dist_to_sphere_center_squared: f64,
  dist_to_sphere_perpendicular_squared: f64,
}
impl Intersection for SphereIntersection {
  fn point(&self) -> Vector {
    let d = (self.radius_squared
      - (self.dist_to_sphere_center_squared - self.dist_to_sphere_perpendicular_squared))
      .sqrt();

    self.ray_direction.normalized() * (self.dist_to_sphere_perpendicular_squared.sqrt() - d)
  }
}

// How can we generalize this? We don't care about `material` here, but we have
// to specify the material type for every implementation :(
impl IntersectsWithRay<SphereIntersection> for Sphere {
  fn intersects(&self, ray: &Ray) -> Option<SphereIntersection> {
    let ray_origin_to_sphere_center = self.center - ray.origin;
    let dist_to_sphere_center_squared = ray_origin_to_sphere_center.length_squared();
    let dist_to_sphere_perpendicular_squared =
      ray_origin_to_sphere_center.dot(&ray.direction).powi(2);

    if (dist_to_sphere_center_squared - dist_to_sphere_perpendicular_squared) < self.radius_squared
    {
      return Some(SphereIntersection {
        radius_squared: self.radius,
        ray_direction: ray.direction,
        dist_to_sphere_center_squared,
        dist_to_sphere_perpendicular_squared,
      });
    }
    None
  }
}

impl Material for Sphere {
  fn color_at<I>(&self, intersection: &I) -> HDRColor
  where
    I: Intersection,
  {
    let point = intersection.point();
    HDRColor {
      r: ((1.0 + (point.x * 2.0).sin()) / 2.0) as f32,
      g: ((1.0 + (point.y * 2.0).cos()) / 2.0) as f32,
      b: ((1.0 + (point.z * 2.0).sin()) / 2.0) as f32,
    }
  }
}
