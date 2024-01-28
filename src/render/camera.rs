use crate::{Vec3, Color3, global};
use crate::{Ray};
use crate::{Interval};
use crate::{global::Parameters};
use crate::{HitRecord, Hittable, HittableList};
use std::fs;
use std::io::Write;
use rand::Rng;
use std::f64::consts::PI;

#[derive(Default)]
pub struct Camera {
    center : Vec3,
    pixel00_loc : Vec3,
    pixel_delta_u : Vec3,
    pixel_delta_v : Vec3,
    aspect_ratio : f64,
    defocus_disk_u : Vec3, //Defocus disk horizontal radius
    defocus_disk_v : Vec3, //Defocus disk vertical radius

    //Camera frame basis vectors
    u : Vec3,
    v : Vec3, 
    w : Vec3,

    //Rendering options
    pub image_width : u32,
    pub image_height : u32,
    pub samples_per_pixel : u32,
    pub max_depth : u32,
    pub fov : f64,

    //Look at transform vectors
    pub eye : Vec3,
    pub target : Vec3, 
    pub up : Vec3,

    //Depth of field parameters
    pub defocus_angle : f64, //Variation angle of rays through each pixel
    pub focus_distance : f64, //Focus distance of camera
}


impl Camera {
    pub fn new(aspect_ratio : f64, width : u32, samples : u32, depth : u32, field_of_view : f64, e : Vec3, t : Vec3, u : Vec3, defocus_a : f64, focus_d : f64) -> Self {
        Self {
            center : e,
            aspect_ratio : aspect_ratio,
            image_width : width,
            image_height : ((width as f64)/aspect_ratio).max(1.0) as u32,
            samples_per_pixel : samples,
            max_depth : depth,
            fov : field_of_view, 
            eye : e,
            target : t, 
            up : u,
            defocus_angle : defocus_a,
            focus_distance : focus_d,
            ..Default::default()
        }
    }

    fn ray_color(ray : &Ray, depth : u32, world : &mut Box<dyn Hittable>) -> Color3 {
        let mut rec : HitRecord = HitRecord::default();
        
        if depth <= 0 {
            return Color3::new(0.0,0.0,0.0);
        }

        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered : Ray = Ray::default();
            let mut attenuation : Color3 = Color3::default();

            if(rec.material.scatter(ray, &rec, &mut attenuation, &mut scattered)){
                return attenuation * Self::ray_color(&scattered, depth-1, world)
            }

            return Color3::default();
        }


        let unit_direction : Vec3 = ray.dir.unit_vector();
        let a : f64 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-a) * Color3::new(1.0,1.0,1.0) + a * Color3::new(0.5, 0.7, 1.0);
    }
    
    pub fn render(&mut self, params : &mut global::Parameters, world : &mut Box<dyn Hittable>){
        self.initialize(params);

        let mut image : fs::File = fs::File::create("image.ppm").expect("Unable to create file");
        writeln!(image, "P3\n{} {}\n255\n", self.image_width, self.image_height).expect("Failed to write data.");


        let mut ray : Ray = Ray::default();
        for j in 0..self.image_height {
            println!("Scanlines Remaining: {}\n", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color : Color3 = Color3::default();
                for sample in 0..self.samples_per_pixel {
                    self.get_ray(i, j, &mut ray);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }

                pixel_color.write_color(&mut image, self.samples_per_pixel);
            }
        }
    }

    fn get_ray(&self, i : u32, j : u32, ray : &mut Ray) {
        let pixel_center : Vec3 = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample : Vec3 = pixel_center + self.pixel_sample_square();
        let ray_origin : Vec3 = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction : Vec3 = pixel_sample - ray_origin;
        
        ray.origin = ray_origin;
        ray.dir = ray_direction;
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut RNG : rand::rngs::ThreadRng = rand::thread_rng();
        let px = -0.5 * RNG.gen::<f64>(); 
        let py = -0.5 * RNG.gen::<f64>();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }

    fn initialize(&mut self, params : &mut global::Parameters) {
        //FOV and viewport stuff
        let theta : f64 = self.fov * (PI / 180.0);
        let h : f64 = (theta/2.0).tan();
        params.viewport_height = 2.0 * h * self.focus_distance;
        params.viewport_width = params.viewport_height * ((self.image_width as f64)/ (self.image_height as f64));

        //Calculate the basis vectors
        self.w = (self.eye - self.target).unit_vector();
        self.u = Vec3::cross(&self.up, &self.w).unit_vector();
        self.v = Vec3::cross(&self.w, &self.u);

        //Calculate the viewport vectors
        let viewport_u : Vec3 = self.u * params.viewport_width;
        let viewport_v : Vec3 = self.v.negate() * params.viewport_height;
        
        //Calculate the horizontal and vertical delta vectors per pixel
        self.pixel_delta_u  = viewport_u / params.width as f64;
        self.pixel_delta_v = viewport_v / params.height as f64;

        //Calculate location of the upper left pixel
        let viewport_upper_left = self.center - (self.focus_distance * self.w) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    
        //Calculate the camera defocus disk basis vectors
        let defocus_radius : f64 = self.focus_distance * ((self.defocus_angle/2.0) * (PI/180.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
}