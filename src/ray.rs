use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
  pub origin: Vector,
  pub direction: Vector,
}
