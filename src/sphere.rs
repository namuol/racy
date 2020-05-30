use crate::intersection::*;
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

pub struct SphereIntersection {}
impl Intersection for SphereIntersection {
  fn point() -> Vector {
    todo!()
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
      return Some(SphereIntersection {});
    }
    None
  }
}
