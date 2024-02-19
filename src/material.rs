use na::Vector3;
use crate::color::RGB;
use crate::ray::Ray;
use crate::scene::HitRecord;
use crate::utils::{rand_unit_vector, NearZero, reflect, refract, rand};

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, RGB)>;
}

#[derive(Default)]
pub struct Lambertian {
    pub albedo: RGB,
}

impl Lambertian {
    pub fn new(color: RGB) -> Self {
        Self { albedo: color }
    }
}

#[derive(Default)]
pub struct Metal {
    pub albedo: RGB,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: RGB, fuzz: f64) -> Self {
        Self { albedo: color, fuzz }
    }
}

#[derive(Default)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Use Shlicks approximation for reflectance
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Ray, RGB)> {
        let mut direction = (hit.normal + rand_unit_vector()) as Vector3<f64>;
        // Account for when random vector subtracts the normal to zero
        if direction.is_near_zero() {
            direction = hit.normal;
        }

        let bounce_ray = Ray::new(hit.p, direction);
        Some((bounce_ray, self.albedo))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, RGB)> {
        let reflected = reflect(&ray.dir.normalize(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * rand_unit_vector());
        if scattered.dir.dot(&hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, RGB)> {
        let refraction_ratio = if hit.front { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_direction = ray.dir.normalize();

        let cos_theta = f64::min((-unit_direction).dot(&hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let can_refract = refraction_ratio * sin_theta <= 1.0;
        let direction = if !can_refract || self.reflectance(cos_theta, refraction_ratio) > rand() {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };
        Some((Ray::new(hit.p, direction), RGB::white()))
    }
}