use nalgebra::{Vector3, clamp};
use std::convert::From;
use std::io::{Result, Write};
use std::ops::Mul;
use crate::utils::{gamma_correct, rand, rand_range};

#[derive(Copy, Clone, Debug, Default)]
pub struct RGB(pub f64, pub f64, pub f64);

unsafe impl Sync for RGB {}
unsafe impl Send for RGB {}

impl RGB {
    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub fn random() -> Self {
        Self(rand(), rand(), rand())
    }

    pub fn rand_range(min: f64, max: f64) -> Self {
        Self(rand_range(min, max), rand_range(min, max), rand_range(min, max))
    }

    pub fn write(&self, samples_per_pixel: u32, writer: &mut dyn Write) -> Result<()> {
        let (r, g, b) = (self.0, self.1, self.2);
        let scale = 1.0 / samples_per_pixel as f64;

        let result_r = gamma_correct(r * scale);
        let result_g = gamma_correct(g * scale);
        let result_b = gamma_correct(b * scale);

        let rint = (256.0 * clamp(result_r, 0.0, 0.999)) as u8;
        let gint = (256.0 * clamp(result_g, 0.0, 0.999)) as u8;
        let bint = (256.0 * clamp(result_b, 0.0, 0.999)) as u8;
        write!(writer, "{} {} {}\n", rint, gint, bint)
    }
}

impl From<Vector3<f64>> for RGB {
    fn from(point: Vector3<f64>) -> Self {
        Self(point.x, point.y, point.z)
    }
}

impl Mul<f64> for RGB {
    type Output = RGB;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl Mul for RGB {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}