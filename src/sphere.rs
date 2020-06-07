use crate::material::Material;
use crate::ray::Ray;
use crate::scene::Renderable;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Sphere {
  pub center: Vector,
  pub radius: f64,
  pub radius_squared: f64,
  pub material: &'static dyn Material,
}

impl Sphere {
  pub fn new(center: Vector, radius: f64, material: &'static dyn Material) -> Self {
    Sphere {
      center,
      radius,
      radius_squared: radius * radius,
      material,
    }
  }
}

impl Renderable for Sphere {
  fn intersects(&self, ray: &Ray) -> Option<f64> {
    // ```text
    //                      , - ~ ~ ~ - ,
    //                  , '               ' ,
    //                ,                       ,
    //               ,                         ,
    //              , self.       self.origin   ,
    //              ,<--radius--->*             ,
    //              ,             |<-y          ,
    //               ,            |            ,
    //  *-------------*<----x---->*-----------*-------->
    //  ^ray.origin   ^t1         ^t       , '^t2
    //                    ' - , _ _ _ ,  '
    // ```
    //
    // Our goal here is to determine whether our ray intersects the circle.
    //
    // We don't need to calculate the _exact_ intersection point(s) yet, we just
    // want to quickly say yes or no (and provide any precomputed info for
    // calculating the actual intersection point for anyone who cares).
    //
    // From the diagram above, it is apparent that if our `y` length is greater
    // than the radius of our circle, we cannot be intersecting.
    //
    // We already have `self.radius`, but how can we calculate `y`?
    //
    // Here's how:
    //
    // 1. First, we draw a vector (`to_center`) from `ray.origin` to
    //    `self.origin`:
    //
    // ```text
    //                        __*<-self.origin
    //      to_center   __..""  | ↑
    //         ↓__..--""        | y
    //   __.--""                | ↓
    //  *-----------------------*-------------->
    //  ^ray.origin ------t----→|  t = ray.direction.dot(to_center)
    // ```
    let to_center = self.center - ray.origin;

    // 2. Next, we take the dot product of this vector-to-our-origin and our
    //    original ray's directional vector. This will give us length `t`.
    let t = ray.direction.dot(&to_center);
    // If `t` is negative, our ray is pointing away from our sphere. This means
    // we can leave early, and in fact we _must_ leave early, otherwise our
    // calculation of `y_squared` will be referring to a sphere in the wrong
    // direction!
    // if t < 0.0 {
    //   return None;
    // }

    // 3. Finally, if we scale our `ray.direction` by `t` (multiply), and
    //    subtract our centerpoint, we get a vector with the length `y`.
    //
    // To avoid a somewhat costly `sqrt` call, we can use `length_squared` to
    // get `y_squared`, and compare that to our `radius_squared`.
    let y_squared = ((ray.direction * t) - to_center).length_squared();

    // If `y_squared` is greater than `radius_squared`, we know we cannot
    // intersect with our sphere.
    if y_squared > self.radius_squared {
      return None;
    }

    // Our goal is to determine a point (vector) where our ray _first_
    // intersects our sphere.
    //
    // To do this we must:
    //
    // 1. Determine the length `x`
    // 2. Subtract this length from `t` and scale our `ray.direction` by it to
    //    determine our intersection point
    //
    // Recall the formula for a circle:
    //
    // - x^2 + y^2 = radius^2
    //
    // We can solve for `x`:
    //
    // - x^2 = radius^2 - y^2
    // - x = sqrt(radius^2 - y^2)
    let x = (self.radius_squared - y_squared).sqrt();

    let t0 = t - x;
    let t1 = t + x;

    // If one of our intersection points is negative, our ray's origin is inside
    // our sphere
    if t0 < 0.0 {
      // If both `t`s are negative, the intersections are occuring "behind" the
      // ray
      if t1 < 0.0 {
        return None;
      }
      // ...otherwise if only one intersection is positive, then we know this
      // must be the intersection point inside the sphere
      return Some(t1);
    }

    // If one of our intersection points is negative, our ray's origin is inside
    // our sphere
    if t1 < 0.0 {
      // If both `t`s are negative, the intersections are occuring "behind" the
      // ray
      if t0 < 0.0 {
        return None;
      }
      // ...otherwise if only one intersection is positive, then we know this
      // must be the intersection point inside the sphere
      return Some(t0);
    }

    // If both intersection points are positive, we want the smaller of the two
    // since that is closest to our ray origin:
    Some(t0.min(t1))
  }

  fn normal(&self, point: &Vector) -> Vector {
    // The normal at this intersection point can be determined by drawing a
    // vector from our sphere's center to our intersection point and normalizing
    // it.
    let mut normal = point - self.center;
    normal.normalize();
    normal
  }

  fn material(&self) -> &dyn Material {
    self.material
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::material::MIRROR;

  #[test]
  fn direct_at_sphere() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 4.0,
      },
      1.0,
      &MIRROR,
    );

    let ray = Ray {
      origin: Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      direction: Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
      },
    };

    match sphere.intersects(&ray) {
      None => panic!("Expected an intersection to occur, but got None"),
      Some(t) => assert_eq!(t, 3.0),
    }
  }

  #[test]
  fn inside_sphere_at_center() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      1.0,
      &MIRROR,
    );

    // We test 1000 random rays out from the center; they should always be equal
    // to the sphere's radius, since the ray is located at the exact center of
    // the sphere.
    for _ in 0..1000 {
      let ray = Ray {
        origin: Vector {
          x: 0.0,
          y: 0.0,
          z: 0.0,
        },
        direction: Vector::random_norm(),
      };
      match sphere.intersects(&ray) {
        None => panic!("Expected an intersection to occur, but got None"),
        Some(t) => assert_eq!(t, sphere.radius),
      }
    }
  }

  #[test]
  fn inside_sphere_off_center() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      1.0,
      &MIRROR,
    );

    // We test 1000 random rays out from the center; they should always be equal
    // to the sphere's radius, since the ray is located at the exact center of
    // the sphere.
    let ray = Ray {
      origin: Vector {
        x: 0.0,
        y: 0.0,
        z: 0.5,
      },
      direction: Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
      },
    };
    match sphere.intersects(&ray) {
      None => panic!("Expected an intersection to occur, but got None"),
      Some(t) => assert_eq!(t, 0.5),
    }

    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      1.0,
      &MIRROR,
    );

    // We test 1000 random rays out from the center; they should always be equal
    // to the sphere's radius, since the ray is located at the exact center of
    // the sphere.
    let ray = Ray {
      origin: Vector {
        x: 0.0,
        y: 0.0,
        z: -0.5,
      },
      direction: Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
      },
    };
    match sphere.intersects(&ray) {
      None => panic!("Expected an intersection to occur, but got None"),
      Some(t) => assert_eq!(t, 1.5),
    }
  }
}
