use crate::math::vec3::{Vec3, Color3};
use crate::math::ray::Ray;
use crate::math::interval::Interval;
use crate::shapes::material::Material;

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub p : Vec3,
    pub normal : Vec3,
    pub material : Material,
    pub t : f64,
    pub front_face : bool
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, interval : Interval, hit_record : &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects : Vec<Box<dyn Hittable>>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray : &Ray, outward_normal : &Vec3){
        self.front_face = Vec3::dot(&ray.dir, outward_normal) < 0.0;
        if(self.front_face){
            self.normal = *outward_normal;
        }
        else {
            self.normal = -1.0 * (*outward_normal);
        }
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {objects : Vec::new()}
    }

    pub fn add(&mut self, object : Box<dyn Hittable>){
        self.objects.push(object);
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray : &Ray, interval : Interval, hit_record : &mut HitRecord) -> bool {
        let mut temp_rec : HitRecord = HitRecord::default();
        let mut hit_anything : bool = false;
        let mut closest_so_far : f64 = interval.max;

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(interval.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                
                //Clone the hit record values 
                *hit_record = temp_rec;
            }
        }

        return hit_anything;
    }
}

