mod color;
mod image;
mod ray;
mod scene;
mod utils;
mod camera;
mod material;

use std::f64::consts::PI;
use color::RGB;
use image::{Image};
use ray::Ray;
use scene::{Sphere};
use material::{Lambertian};

extern crate nalgebra as na;
use na::{point, vector};
use std::io::Result;
use std::sync::Arc;
use crate::camera::{Camera};
use crate::material::{Dielectric, Metal};
use crate::scene::Scene;
use crate::utils::{rand, rand_range};

fn main() -> Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let w = 1200;
    let samples = 50;
    let max_bounces= 10;

    let scene = final_scene();
    let mut camera = Camera::new(
        w,
        aspect_ratio,
        samples,
        max_bounces,
        20.0,
        point![12.0, 2.0, 3.0],
        point![0.0, 0.0, 0.0],
        vector![0.0, 1.0, 0.0],
        0.6,
        10.0
    );

    // Render
    let renderer = camera.renderer();
    let image = renderer.render_parallel(scene.clone());
    eprintln!("Done");
    let mut file = std::fs::File::create("image.ppm")?;
    let _ = image.save(&mut file).unwrap();
    Ok(())
}

fn setup_scene() -> Scene {
    let mut scene = Scene::new();
    let material_ground = Arc::new(Lambertian::new(RGB(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(RGB(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(RGB(0.8, 0.6, 0.2), 0.0));

    scene.add(Arc::new(Sphere {
        center: point![0.0, -100.5, -1.0],
        radius: 100.0,
        material: material_ground.clone()
    }));
    scene.add(Arc::new(Sphere {
        center: point![0.0, 0.0, -1.0],
        radius: 0.5,
        material: material_center.clone()
    }));
    scene.add(Arc::new(Sphere {
        center: point![-1.0, 0.0, -1.0],
        radius: 0.5,
        material: material_left.clone()
    }));
    scene.add(Arc::new(Sphere {
        center: point![1.0, 0.0, -1.0],
        radius: 0.5,
        material: material_right.clone()
    }));
    scene
}

fn setup_scene2() -> Scene {
    let mut scene = Scene::new();

    let R = (PI / 4.0).cos();
    let mat_left = Arc::new(Lambertian::new(RGB(0.0, 0.0, 1.0)));
    let mat_right = Arc::new(Lambertian::new(RGB(1.0, 0.0, 0.0)));

    scene.add(Arc::new(Sphere {
        center: point![-R, 0.0, -1.0],
        radius: R,
        material: mat_left.clone()
    }));
    scene.add(Arc::new(Sphere {
        center: point![R, 0.0, -1.0],
        radius: R,
        material: mat_right.clone()
    }));
    scene
}

fn final_scene() -> Arc<Scene> {
    let mut scene = Scene::new();
    let ground_material = Arc::new(Lambertian::new(RGB(0.5, 0.5, 0.5)));

    scene.add(Arc::new(Sphere {
        center: point![0.0, -1000.0, 0.0],
        radius: 1000.0,
        material: ground_material.clone()
    }));

    for a in -5..5 {
        for b in -5..5 {
            let af = a as f64;
            let bf = b as f64;
            let choose_mat = rand();
            let center = point![af + 0.9 * rand(), 0.2, bf + 0.9 * rand()];

            if (center - point![4.0, 0.2, 0.0]).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = RGB::random() * RGB::random();
                    scene.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian::new(albedo))
                    }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = RGB::rand_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    scene.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(albedo, fuzz))
                    }));
                } else {
                    // glass
                    scene.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric::new(1.5))
                    }));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    scene.add(Arc::new(Sphere {
        center: point![0.0, 1.0, 0.0],
        radius: 1.0,
        material: mat1.clone()
    }));

    let mat2 = Arc::new(Lambertian::new(RGB(0.4, 0.2, 0.1)));
    scene.add(Arc::new(Sphere {
        center: point![-4.0, 1.0, 0.0],
        radius: 1.0,
        material: mat2.clone()
    }));

    let mat3 = Arc::new(Metal::new(RGB(0.7, 0.6, 0.5), 0.0));
    scene.add(Arc::new(Sphere {
        center: point![4.0, 1.0, 0.0],
        radius: 1.0,
        material: mat3.clone()
    }));

    Arc::new(scene)
}


#[cfg(test)]
mod test {
    use approx::{assert_relative_eq, relative_eq};
    use na::{vector, Vector3};
    use crate::utils::rand_unit_vector;

    #[test]
    fn test_fn() {

    }
}