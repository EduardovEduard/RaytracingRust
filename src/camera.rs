use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use na::{Point3, vector, Vector3};
use rayon::prelude::*;
use crate::image::{PPM};
use crate::ray::Ray;
use crate::RGB;
use crate::scene::{Hittable, Scene};
use crate::utils::{degrees_to_radians, INF, rand, rand_in_unit_disk};

#[derive(Copy, Clone, Default)]
struct Pixel {
    i: usize,
    j: usize,
    color: RGB,
}

pub struct Renderer {
    render_width: usize,
    render_height: usize,
    samples_per_pixel: u32,
    max_bounces: u32,
    camera: Arc<Camera>
}

impl Renderer {
    pub fn render_parallel(&self, scene: Arc<Scene>) -> Box<PPM> {
        let mut image = Box::new(PPM::new(self.render_width, self.render_height, self.samples_per_pixel));
        let counter = AtomicUsize::new(0);
        let pixels: Vec<RGB> = (0..self.render_height).clone().into_par_iter().flat_map(|i| {
            eprintln!("Scanlines remaining: {}", self.render_height - i);
            let s = scene.clone();
            (0..self.render_width).clone().into_par_iter().map(move |j| {
                let mut sample_result = Vector3::<f64>::zeros();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.camera.sample_ray(i, j);
                    let color = ray_color(&ray, self.max_bounces, &s);
                    sample_result += vector![color.0, color.1, color.2];
                }

                RGB::from(sample_result)
            })
        }).collect::<Vec<_>>();

        (0..self.render_height).for_each(|i| {
            (0..self.render_width).for_each(|j| {
                image[(i, j)] = pixels[i * self.render_width + j];
            });
        });

        image
    }
}

#[derive(Default, Clone)]
pub struct Camera {
    pub render_width: usize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub fov_degrees: f64,
    pub lookfrom: Point3<f64>,
    pub lookat: Point3<f64>,
    pub vup: Vector3<f64>,
    pub defocus_angle_degrees: f64,
    pub focus_dist: f64,

    render_height: usize, // Rendered image height
    center: Point3<f64>, // Camera center
    pixel00_loc: Point3<f64>, // Location of pixel (0, 0)
    pixel_delta_u: Vector3<f64>, // Offset to pixel to the right
    pixel_delta_v: Vector3<f64>, // Offset to pixel below

    // Camera frame basis vectors
    u: Vector3<f64>, // right
    v: Vector3<f64>, // up
    w: Vector3<f64>, // backwards

    defocus_disk_u: Vector3<f64>, // Defocus disk horizontal radius
    defocus_disk_v: Vector3<f64> // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        width: usize,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_bounces: u32,
        fov: f64,
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        vup: Vector3<f64>,
        defocus_angle_degrees: f64,
        focus_dist: f64
    ) -> Self {
        Self {
            render_width: width,
            aspect_ratio,
            samples_per_pixel,
            max_bounces,
            fov_degrees: fov,
            lookfrom,
            lookat,
            vup,
            defocus_angle_degrees,
            focus_dist,
            ..Default::default()
        }
    }

    pub fn renderer(&mut self) -> Renderer {
        self.initialize();
        Renderer {
            render_width: self.render_width,
            render_height: self.render_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            camera: Arc::new(self.clone())
        }
    }

    // TODO Remove mut and use interior mutability (RefCell)
    pub fn render(&mut self, scene: &Scene) -> Box<PPM> {
        self.initialize();

        let mut image = Box::new(PPM::new(self.render_width, self.render_height, self.samples_per_pixel));
        for i in 0..self.render_height {
            eprintln!("Scanlines remaining: {}", self.render_height - i);
            for j in 0..self.render_width {
                let mut sample_result = Vector3::<f64>::zeros();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.sample_ray(i, j);
                    let color = ray_color(&ray, self.max_bounces, &scene);
                    sample_result += vector![color.0, color.1, color.2];
                }
                image[(i, j)] = sample_result.into();
            }
        }
        image
    }

    fn sample_ray(&self, i: usize, j: usize) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
        let pixel_center =
            self.pixel00_loc + (j as f64 * self.pixel_delta_u) + (i as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle_degrees <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = rand_in_unit_disk();
        return self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vector3<f64> {
        let px = -0.5 + rand();
        let py = -0.5 + rand();
        return px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn initialize(&mut self) {
        self.render_height = (self.render_width as f64 / self.aspect_ratio) as usize;
        if self.render_height < 1 {
            self.render_height = 1;
        }
        println!("Image size: W:{}, H:{}", self.render_width, self.render_height);
        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.fov_degrees);
        // height of camera field of view
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.render_width as f64) / (self.render_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = (self.vup.cross(&self.w)).normalize();
        self.v = self.w.cross(&self.u);

        println!(
            "Initialized viewport: W:{}, H:{}",
            viewport_width, viewport_height
        );

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.render_width as f64;
        self.pixel_delta_v = viewport_v / self.render_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5f64 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle_degrees / 2.0).tan());
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
}

fn ray_color(ray: &Ray, depth: u32, scene: &Scene) -> RGB {
    if depth <= 0 {
        return RGB::default();
    }

    // Reduce the probability of falling inside the surface due to fp errors
    let mint = 0.001;
    if let Some(hit) = scene.hit(&ray, mint..INF) {
        return match hit.material.scatter(&ray, &hit) {
            Some((scattered, attenuation)) => {
                attenuation * ray_color(&scattered, depth - 1, scene)
            },
            None => RGB::default()
        }
    }

    // Sky
    let unit = ray.dir.normalize();
    let a = 0.5 * (unit.y + 1.0);
    let blue = vector![0.5, 0.7, 1.0];
    let white = vector![1.0, 1.0, 1.0];
    white.lerp(&blue, a).into()
}
