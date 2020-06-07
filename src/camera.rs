// Quick port from *coffeescript* 😂
// class Camera
//   constructor: (@eye, @fovy, @scr_w, @scr_h) ->
//     @fovx = (@scr_w/@scr_h) * @fovy
//
//     # Precompute ray position variables for quick ray generation in "getRayFromUV":
//     @xstart = -0.5*@fovx/45.0
//     @ystart = 0.5*@fovy/45.0
//     @xmult = (@fovx/45.0) / @scr_w
//     @ymult = -(@fovy/45.0) / @scr_h
//     @setAng(0)
//
//   setAng: (v) ->
//     @ang = v
//     @look = new Vector(-Math.sin(@ang), 0, -Math.cos(@ang)).normal()
//     @perp = new Vector(-Math.sin(@ang + Math.PI/2), 0, -Math.cos(@ang + Math.PI/2)).normal()
//
//   getRayFromUV: (u, v)->
//     p = @look.sub((@perp.mul((@xstart + u*@xmult))))
//     return new Vector(p.x, @ystart + v*@ymult, p.z).normal()
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Camera {
  pub eye: Vector,
  pub look: Vector,
  perp: Vector,
  pub angle: f64,
  pub screen_width: u32,
  pub screen_height: u32,
  xstart: f64,
  ystart: f64,
  xmult: f64,
  ymult: f64,
}

impl Camera {
  pub fn new(eye: Vector, fovy: f64, screen_width: u32, screen_height: u32) -> Self {
    let fovx = (screen_width as f64 / screen_height as f64) * fovy;
    let xstart = -0.5 * fovx / 45.0;
    let ystart = 0.5 * fovy / 45.0;
    let xmult = (fovx / 45.0) / screen_width as f64;
    let ymult = -(fovy / 45.0) / screen_height as f64;

    let mut camera = Camera {
      eye,
      look: Vector::new(),
      perp: Vector::new(),
      angle: 0.0,
      screen_width,
      screen_height,
      xstart,
      ystart,
      xmult,
      ymult,
    };

    camera.set_angle(0.0);

    camera
  }

  pub fn set_angle(&mut self, angle: f64) -> &mut Camera {
    use std::f64::consts::PI;

    self.angle = angle;

    self.look.x = -(angle.sin());
    self.look.y = 0.0;
    self.look.z = -(angle.cos());
    self.look.normalize();

    self.perp.x = -(angle + (PI / 2.0)).sin();
    self.perp.y = 0.0;
    self.perp.z = -(angle + (PI / 2.0)).cos();
    self.perp.normalize();

    self
  }

  pub fn get_ray_from_uv(&self, u: f32, v: f32) -> Ray {
    let p = self.look - (self.perp * (self.xstart + (u as f64 * self.xmult)));

    let mut direction = Vector {
      x: p.x,
      y: self.ystart + (v as f64 * self.ymult),
      z: p.z,
    };

    direction.normalize();

    Ray {
      origin: self.eye,
      direction,
    }
  }
}
