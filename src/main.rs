mod global;
mod math;
mod shapes;
mod render;


use std::f64::consts::PI;

use math::vec3::{Color3, Vec3, Point3};
use math::ray::Ray;
use math::interval::Interval;
use shapes::hittable::{HitRecord, Hittable, HittableList};
use shapes::material::{Material};
use shapes::sphere::Sphere;
use render::camera::Camera;


fn main() {
    let mut parameters : global::Parameters = global::Parameters::new(400, 225);
    let mut camera : Camera = Camera::new(16.0/9.0, 400, 500, 50, 20.0, Vec3::new(-2.0, 2.0, 1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0));

    //Create materials
    let material_ground: Material = Material::Lambertian { albedo: Color3::new(0.8, 0.8, 0.0) };
    let material_center: Material = Material::Lambertian { albedo: Color3::new(0.1,0.2,0.5) }; 
    let material_left: Material = Material::Diaelectric { index_of_refraction: 1.5 };
    let material_right: Material = Material::Metal { albedo: Color3::new(0.8, 0.6, 0.2), fuzz: 0.0 };

    //World
    let mut world : HittableList = HittableList::new();
    let mut world_box : Box<dyn Hittable>;
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    world_box = Box::new(world); //Last thing you should do!

    camera.render(&mut parameters, &mut world_box);    
}
