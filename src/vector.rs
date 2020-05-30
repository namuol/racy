// Quick port from *coffeescript* 😂
// class Vector
//   constructor: (@x, @y, @z) ->
//   add: (v) ->
//     return new Vector(@x+v.x, @y+v.y, @z+v.z)
//   sub: (v) ->
//     return new Vector(@x-v.x, @y-v.y, @z-v.z)
//   mul: (s) ->
//     return new Vector(@x*s,@y*s,@z*s)
//   normal: ->
//     mag = Math.sqrt(@x*@x + @y*@y + @z*@z)
//     @x /= mag
//     @y /= mag
//     @z /= mag
//     @
//   len: ->
//     return Math.sqrt @x*@x + @y*@y + @z*@z

use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector {
  Vector {
    x: a.x + b.x,
    y: a.y + b.y,
    z: a.z + b.z,
  }
});

impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector {
  Vector {
    x: a.x - b.x,
    y: a.y - b.y,
    z: a.z - b.z,
  }
});

impl_op_ex!(*|a: &Vector, b: f64| -> Vector {
  Vector {
    x: a.x * b,
    y: a.y * b,
    z: a.z * b,
  }
});

impl_op_ex!(/|a: &Vector, b: f64| -> Vector {
  Vector {
    x: a.x / b,
    y: a.y / b,
    z: a.z / b,
  }
});

impl_op_ex!(+=|a: &mut Vector, b: &Vector| {
  a.x += b.x;
  a.y += b.y;
  a.z += b.z;
});

impl_op_ex!(-=|a: &mut Vector, b: &Vector| {
  a.x -= b.x;
  a.y -= b.y;
  a.z -= b.z;
});

impl_op_ex!(*=|a: &mut Vector, b: f64| {
  a.x *= b;
  a.y *= b;
  a.z *= b;
});

impl_op_ex!(/=|a: &mut Vector, b: f64| {
  a.x /= b;
  a.y /= b;
  a.z /= b;
});

impl Vector {
  pub fn new() -> Self {
    Vector {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn normalize(&mut self) -> &mut Self {
    // TODO: The borrow-checker doesn't like this:
    // self /= self.length();

    let length = self.length();
    self.x /= length;
    self.y /= length;
    self.z /= length;

    self
  }

  pub fn normalized(&self) -> Self {
    let length = self.length();
    self / length
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn equal_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let b = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert_eq!(a, b);
  }

  #[test]
  fn add_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let b = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert_eq!(
      a + b,
      Vector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
      }
    );
  }

  #[test]
  fn sub_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let b = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert_eq!(
      a - b,
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      }
    );
  }

  #[test]
  fn mul_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(
      a * 3.0,
      Vector {
        x: 3.0,
        y: 6.0,
        z: 9.0,
      }
    );
  }

  #[test]
  fn add_assign_operator() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a += Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(
      a,
      Vector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
      }
    )
  }

  #[test]
  fn sub_assign_operator() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a -= Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(
      a,
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      }
    )
  }

  #[test]
  fn mul_assign_operator() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a *= 2.0;

    assert_eq!(
      a,
      Vector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
      }
    )
  }

  #[test]
  fn div_assign_operator() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a /= 2.0;

    assert_eq!(
      a,
      Vector {
        x: 0.5,
        y: 1.0,
        z: 1.5,
      }
    )
  }

  #[test]
  fn length_squared() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(a.length_squared(), 14.0);
  }

  #[test]
  fn length() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(a.length(), (a.length_squared()).sqrt());
  }

  #[test]
  fn normalize() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a.normalize();

    assert_eq!(a.length(), 1.0);
  }

  #[test]
  fn normalized() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(a.normalized().length(), 1.0);
  }
}
