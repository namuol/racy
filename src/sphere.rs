use crate::intersection::*;
use crate::material::HDRColor;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Sphere {
  pub center: Vector,
  pub radius: f64,
  pub radius_squared: f64,
}

impl Sphere {
  pub fn new(center: Vector, radius: f64) -> Self {
    Sphere {
      center,
      radius,
      radius_squared: radius * radius,
    }
  }
}

pub struct SphereIntersection {
  radius_squared: f64,
  y_squared: f64,
  t: f64,
  ray: Ray,
  sphere_center: Vector,
}

impl Intersection for SphereIntersection {
  fn point(&self) -> Vector {
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
    let x = (self.radius_squared - self.y_squared).sqrt();

    self.ray.origin + self.ray.direction * (self.t - x)
  }

  fn normal(&self) -> Vector {
    // The normal at this intersection point can be determined by drawing a
    // vector from our sphere's center to our intersection point and normalizing
    // it.
    let mut normal = self.point() - self.sphere_center;
    normal.normalize();
    normal
  }

  fn dist_squared(&self) -> f64 {
    (self.ray.origin - self.point()).length_squared()
  }
}

impl IntersectsWithRay<SphereIntersection> for Sphere {
  fn intersects(&self, ray: &Ray) -> Option<SphereIntersection> {
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
    if t < 0.0 {
      return None;
    }

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

    // ...Otherwise, we must be intersecting with our sphere!
    Some(SphereIntersection {
      y_squared,
      radius_squared: self.radius_squared,
      ray: *ray,
      t,
      sphere_center: self.center,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn direct_at_sphere() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      1.0,
    );

    let ray = Ray {
      origin: Vector {
        x: 0.0,
        y: 0.0,
        z: -4.0,
      },
      direction: Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
      },
    };

    match sphere.intersects(&ray) {
      None => panic!("Expected an intersection to occur, but got None"),
      Some(intersection) => {
        assert_eq!(
          intersection.point(),
          Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
          },
          "Intersection point is not what we expected"
        );
      }
    }
  }

  #[test]
  fn direct_at_sphere_2() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 4.0,
      },
      1.0,
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
      Some(intersection) => {
        assert_eq!(
          intersection.point(),
          Vector {
            x: 0.0,
            y: 0.0,
            z: 3.0,
          },
          "Intersection point is not what we expected"
        );
      }
    }
  }

  #[test]
  fn away_from_sphere() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 4.0,
      },
      1.0,
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
        z: -1.0,
      },
    };
    match sphere.intersects(&ray) {
      None => (),
      Some(_) => panic!("Expected no intersection to occur, but we got Some!"),
    }
  }

  #[test]
  fn away_from_sphere_2() {
    let sphere = Sphere::new(
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      1.0,
    );
    let ray = Ray {
      origin: Vector {
        x: 0.0,
        y: 0.0,
        z: -4.0,
      },
      direction: Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
      },
    };
    match sphere.intersects(&ray) {
      None => (),
      Some(_) => panic!("Expected no intersection to occur, but we got Some!"),
    }
  }
}

const POINT_LIGHT: Vector = Vector {
  x: 0.0,
  y: 4.0,
  z: 0.0,
};

const POINT_LIGHT_POWER: f64 = 4.0;

impl Material for Sphere {
  fn color_at<I>(&self, intersection: &I) -> HDRColor
  where
    I: Intersection,
  {
    // For debugging normals:
    //
    // let normal = intersection.normal();
    // return HDRColor {
    //   r: ((1.0 + normal.x) / 2.0) as f32,
    //   g: ((1.0 + normal.y) / 2.0) as f32,
    //   b: (0.5 - normal.z) as f32,
    // };

    // ```text
    //                * <-light.origin
    //                |
    //                |
    //  normal        |
    //  ""..__  theta |     , - ~ ~ ~ - ,
    //        ""..__  | , '               ' ,
    //  *-----------""*                       ,
    //  ^ray.origin   ^point                   '
    //              ,                           ,
    //              ,                           ,
    // ```
    //
    // Let's implement diffuse ("Lambertian") reflection.
    //
    // To do this, all we need is the angle between the light source and our
    // normal.

    // 1. Draw a vector from our intersection point to the light source:
    let to_light = POINT_LIGHT - intersection.point();

    // 2. Use the dot product to calculate theta.cos()
    let theta_cos = to_light.dot(&intersection.normal());

    // 3. We employ the inverse-square law to determine how intense the light
    //    should be:
    let intensity = POINT_LIGHT_POWER / (to_light.length_squared());

    // 4. Finally, we just multiply our lighting intensity by the cosine of the
    //    angle between our normal and the incoming light:
    let illumination = intensity * theta_cos;

    HDRColor {
      r: illumination as f32,
      g: illumination as f32,
      b: illumination as f32,
    }
  }
}
