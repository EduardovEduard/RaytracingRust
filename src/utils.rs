use std::f64::consts::PI;
use na::{vector, Vector3};
use rand::{random, Rng};

pub const INF: f64 = f64::MAX;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub fn rand() -> f64 {
    random::<f64>()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand_in_unit_sphere() -> Vector3<f64> {
    loop {
        let distribution = rand::distributions::Uniform::new(-1.0, 1.0);
        let random = Vector3::<f64>::from_distribution(&distribution, &mut rand::thread_rng());
        if random.norm_squared() < 1.0 {
            return random
        }
    }
}

pub fn rand_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = vector![rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0];
        if p.norm_squared() < 1.0 {
            return p
        }
    }
}

pub fn rand_unit_vector() -> Vector3<f64> {
    rand_in_unit_sphere().normalize()
}

pub fn rand_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = rand_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn gamma_correct(linear: f64) -> f64 {
    linear.sqrt()
}

pub fn reflect(ray: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    ray - 2.0 * ray.dot(&normal) * normal
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min((-uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel

}

pub trait NearZero {
    fn is_near_zero(&self) -> bool;
}

impl NearZero for Vector3<f64> {
    fn is_near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
}