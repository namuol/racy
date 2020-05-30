use crate::intersection::Intersection;

pub trait Material {
  fn color_at<I>(&self, intersection: &I) -> HDRColor
  where
    I: Intersection;
}

#[derive(Copy, Clone)]
pub struct HDRColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Material for HDRColor {
  fn color_at<I>(&self, _: &I) -> HDRColor
  where
    I: Intersection,
  {
    *self
  }
}
