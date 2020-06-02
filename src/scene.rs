use crate::camera::Camera;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Scene {
  pub cam: Camera,
  pub renderables: Vec<Box<dyn Renderable>>,
}

pub trait Renderable: Sync {
  fn intersects(&self, ray: &Ray) -> Option<Vector>;
  fn normal(&self, point: &Vector) -> Vector;
  fn material(&self) -> &dyn Material;
}
