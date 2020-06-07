use crate::camera::Camera;
use crate::material::*;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Light {
  pub center: Vector,
  pub color: HDRColor,
  pub radius: f32,
}

pub struct Scene {
  pub cam: Camera,
  pub renderables: Vec<Box<dyn Renderable>>,
  pub bg_color: HDRColor,
  pub lights: Vec<Light>,
  pub photons: Vec<Light>,
}

#[derive(Copy, Clone)]
pub struct Intersection {
  pub renderable_idx: usize,
  pub t: f64,
  pub depth: u8,
}

impl Scene {
  pub fn cast(&self, ray: &Ray, depth: u8) -> Option<Intersection> {
    let mut maybe_closest_intersection: Option<Intersection> = None;
    let mut renderable_idx = 0;
    for object in &self.renderables {
      match object.intersects(ray) {
        None => (),
        Some(t) => match maybe_closest_intersection {
          None => {
            maybe_closest_intersection = Some(Intersection {
              renderable_idx,
              t,
              depth,
            })
          }
          Some(closest_intersection) => {
            if closest_intersection.t > t {
              maybe_closest_intersection = Some(Intersection {
                renderable_idx,
                t,
                depth,
              })
            }
          }
        },
      }

      renderable_idx += 1;
    }

    maybe_closest_intersection
  }
}

pub trait Renderable: Sync {
  fn intersects(&self, ray: &Ray) -> Option<f64>;
  fn normal(&self, point: &Vector) -> Vector;
  fn material(&self) -> &dyn Material;
}
