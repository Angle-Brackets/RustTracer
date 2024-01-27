use std::fmt;

use rand::Rng;

use crate::{Vec3, Color3, Ray};
use crate::{HitRecord, Hittable, HittableList};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {albedo : Vec3},
    Metal {albedo : Vec3, fuzz : f64}, //Fuzz is the distortion of the reflection, clamped to [0,1]. 0 is a mirror, 1 is very rough reflection.
    Diaelectric {index_of_refraction : f64}
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color3::new(0.5, 0.5, 0.5), // Set default albedo for Lambertian
        }
    }
}

impl Material {
    pub fn scatter(&self, ray_in : &Ray, rec : &HitRecord, attenuation : &mut Color3, scattered : &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction : Vec3 = rec.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                scattered.origin = rec.p;
                scattered.dir = scatter_direction;
                
                *attenuation = *albedo;
                return true;
            }

            Material::Metal { albedo, fuzz } => {
                let reflected : Vec3 = Vec3::reflect(&ray_in.dir.unit_vector(), &rec.normal);
                scattered.origin = rec.p;
                scattered.dir = reflected + fuzz.clamp(0.0, 1.0)*Vec3::random_unit_vector();

                *attenuation = *albedo;
                return Vec3::dot(&scattered.dir, &rec.normal) > 0.0;
            }

            Material::Diaelectric { index_of_refraction } => {
                *attenuation = Color3::new(1.0, 1.0, 1.0);
                let refraction_ratio : f64 = if rec.front_face {1.0/index_of_refraction} else {*index_of_refraction};

                //Before determining if we refract, we need to see if snell's law has a solution, this can be found if the ratio of
                //the refractive indices are > 1, if they are, then we cannot have a solution and must do a pure reflection of the 
                //surface rather than a refraction.
                let unit_direction : Vec3 = ray_in.dir.unit_vector();
                let cos_theta : f64 = Vec3::dot(&unit_direction.negate(), &rec.normal).min(1.0);
                let sin_theta : f64 = (1.0 - cos_theta*cos_theta).sqrt();

                let can_refract : bool = refraction_ratio * sin_theta < 1.0;
                let mut direction : Vec3 = Vec3::default();
                let mut rng : rand::rngs::ThreadRng = rand::thread_rng();

                if can_refract && reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
                    direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
                }

                else {
                    direction = Vec3::reflect(&unit_direction, &rec.normal);
                }

                scattered.origin = rec.p;
                scattered.dir = direction;

                return true;
            }
        }
    }
}

//Private helper functions

//Implementation of the Schlick Approximation for Reflective surfaces
fn reflectance(cosine : f64, ref_idx : f64) -> f64 {
    let mut r0 : f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    return r0 + (1.0 - r0) * (1.0-cosine).powf(5.0);
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Material::Lambertian { albedo } => {
                write!(f, "Albedo: [{}, {}, {}]", albedo[0], albedo[1], albedo[2])
            }

            Material::Metal { albedo, fuzz } => {
                write!(f, "Albedo: {}\nFuzz: {}", albedo, fuzz)
            }

            _ => {
                write!(f, "No print output designed for this material.")
            }
        }
    }
}