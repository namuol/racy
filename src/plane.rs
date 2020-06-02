use crate::material::Material;
use crate::ray::Ray;
use crate::scene::Renderable;
use crate::Vector;

#[derive(Copy, Clone)]
pub struct Plane {
  pub center: Vector,
  normal: Vector,
  material: &'static dyn Material,
}

impl Plane {
  pub fn new(center: Vector, normal: Vector, material: &'static dyn Material) -> Self {
    Plane {
      center,
      normal: normal.normalized(),
      material,
    }
  }
}

impl Renderable for Plane {
  fn intersects(&self, ray: &Ray) -> std::option::Option<Vector> {
    let denominator = self.normal.dot(&ray.direction);
    if denominator.abs() < 0.0001 {
      return None;
    }
    let t = (self.center - ray.origin).dot(&self.normal) / denominator;
    if t < 0.0001 {
      return None;
    }
    Some(ray.direction * t)
  }

  fn normal(&self, _: &Vector) -> Vector {
    self.normal
  }

  fn material(&self) -> &dyn Material {
    self.material
  }
}
