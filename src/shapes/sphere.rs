use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::material::{Material};
use crate::math::vec3::{Vec3, Color3};
use crate::math::ray::Ray;
use crate::math::interval::Interval;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center : Vec3,
    pub radius : f64,
    pub material : Material
}

impl Sphere {
    pub fn new(c : Vec3, r : f64, mat : Material) -> Self {
        Self {center: c, radius : r, material : mat}
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {center : Vec3::default(), radius : 0.0, material : Material::Lambertian { albedo: Color3::new(1.0,0.5,0.5) }}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray : &Ray, interval : Interval, hit_record : &mut HitRecord) -> bool {
        let oc : Vec3 = ray.origin - self.center;
        let a : f64 = ray.dir.length_squared();
        let half_b : f64 = Vec3::dot(&oc, &ray.dir);
        let c : f64 = oc.length_squared() - self.radius*self.radius;
        let discriminant : f64 = half_b*half_b - a*c;

        if(discriminant < 0.0){
            return false;
        }
        
        //Find the nearest root that lies in a close enough range of the position
        let sqrtd : f64 = discriminant.sqrt();
        let mut root : f64 = (-half_b - sqrtd) / a;

        if(!interval.surrounds(root)){
            root = (-half_b + sqrtd) / a;
            if(!interval.surrounds(root)){
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        hit_record.material = self.material;
        let outward_normal : Vec3 = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        return true;


    }
} 