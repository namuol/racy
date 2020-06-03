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
  fn intersects(&self, ray: &Ray) -> Option<f64> {
    let dir = ray.direction.normalized();
    let denominator = self.normal.normalized().dot(&dir);
    if denominator.abs() < 0.0001 {
      return None;
    }
    let d = -self.normal.normalized().dot(&self.center);
    let t = -(self.normal.normalized().dot(&ray.origin) + d) / denominator;
    if t < 0.0001 {
      return None;
    }

    Some(t)
  }

  fn normal(&self, _: &Vector) -> Vector {
    self.normal
  }

  fn material(&self) -> &dyn Material {
    self.material
  }
}
