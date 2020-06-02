use crate::camera::Camera;
use crate::material::*;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Scene {
  pub cam: Camera,
  pub renderables: Vec<Box<dyn Renderable>>,
  pub bg_color: HDRColor,
  pub light_pos: Vector,
  pub light_power: f64,
}

impl Scene {
  pub fn cast(&self, ray: &Ray, depth: u8) -> Option<HDRColor> {
    let mut intersection_obj: Option<(Vector, &dyn Material, Vector, f64)> = None;
    for object in &self.renderables {
      match object.intersects(ray) {
        None => continue,
        Some(this_intersection) => match intersection_obj {
          None => {
            let this_dist_squared = (ray.origin - this_intersection).length_squared();
            intersection_obj = Some((
              this_intersection,
              object.material(),
              object.normal(&this_intersection),
              this_dist_squared,
            ));
          }
          Some((_, _, _, closest_dist_squared)) => {
            let this_dist_squared = (ray.origin - this_intersection).length_squared();
            if this_dist_squared < closest_dist_squared {
              intersection_obj = Some((
                this_intersection,
                object.material(),
                object.normal(&this_intersection),
                this_dist_squared,
              ));
            }
          }
        },
      }
    }

    if let Some((intersection, material, normal, _)) = intersection_obj {
      return Some(material.color_at(&intersection, &normal, &ray, &self, depth));
    }

    None
  }
}

pub trait Renderable: Sync {
  fn intersects(&self, ray: &Ray) -> Option<Vector>;
  fn normal(&self, point: &Vector) -> Vector;
  fn material(&self) -> &dyn Material;
}
