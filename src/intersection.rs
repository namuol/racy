use crate::ray::Ray;
use crate::vector::Vector;

pub trait Intersection {
  fn point(&self) -> Vector;
  fn normal(&self) -> Vector;
  // fn dist(&self) -> f64;
  // fn dist_squared(&self) -> f64;
}

pub trait IntersectsWithRay<I>
where
  I: Intersection,
{
  fn intersects(&self, ray: &Ray) -> Option<I>
  where
    I: Intersection;
}
