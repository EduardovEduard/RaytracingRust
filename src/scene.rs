use std::ops::{Range};
use std::sync::Arc;
use crate::Ray;
use na::{Point3, Vector3};
use crate::material::Material;

pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front: bool,
    pub material: Arc<dyn Material>
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, trange: Range<f64>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, trange: Range<f64>) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.norm_squared(); // ray.dir.dot(&ray.dir);
        let half_b = oc.dot(&ray.dir);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        // Try both roots
        if root <= trange.start || root >= trange.end {
            root = (-half_b + sqrtd) / a;
            if root <= trange.start || root >= trange.end {
                return None;
            }
        }

        let hitpoint = ray.at(root);
        let normal = (hitpoint - self.center) / self.radius;
        let outside = ray.dir.dot(&normal) < 0.0;
        let hit = HitRecord {
            t: root,
            p: hitpoint,
            normal: if outside { normal } else { -normal },
            front: outside,
            material: self.material.clone(),
        };
        return Some(hit);
    }
}

pub struct Scene {
    pub hittables: Vec<Arc<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { hittables: vec![] }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, trange: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = trange.end;
        let mut result = None;
        self.hittables.iter().for_each(|hittable| {
            if let Some(hit) = hittable.hit(ray, trange.start..closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        });
        return result;
    }
}

