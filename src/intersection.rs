use crate::ray::Ray;
use crate::vector::Vector;

pub trait Intersection {
  fn point() -> Vector;
  // fn normal() -> Vector;
}

pub trait IntersectsWithRay<I>
where
  I: Intersection,
{
  fn intersects(&self, ray: &Ray) -> Option<I>
  where
    I: Intersection;
}
