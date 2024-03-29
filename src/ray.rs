extern crate nalgebra as na;
use na::{Point3, Vector3};

#[derive(Default, Debug)]
pub struct Ray {
    pub orig: Point3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Point3<f64>, dir: Vector3<f64>) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.orig + t * self.dir
    }
}
