mod global;
mod math;
mod shapes;
mod render;

use math::vec3::{Color3, Vec3, Point3};
use math::ray::Ray;
use math::interval::Interval;
use shapes::hittable::{HitRecord, Hittable, HittableList};
use shapes::material::{self, Material};
use shapes::sphere::{self, Sphere};
use render::camera::Camera;
use rand::Rng;


fn main() {
    let mut parameters : global::Parameters = global::Parameters::new(400, 225);
    let mut camera : Camera = Camera::new(16.0/9.0, 1200, 500, 50, 20.0, Vec3::new(13.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.6, 10.0);
    let mut rng : rand::rngs::ThreadRng = rand::thread_rng();

    //Create materials
    let material_ground: Material = Material::Lambertian { albedo: Color3::new(0.5, 0.5, 0.5) };
    let scene_focus : Vec3 = Vec3::new(4.0, 0.2, 0.0);

    //World
    let mut world : HittableList = HittableList::new();
    let mut world_box : Box<dyn Hittable>;

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat : f64 = rng.gen::<f64>();
            let center : Vec3 = Vec3::new(a as f64 + 0.9*rng.gen::<f64>(), 0.2, b as f64 + 0.9*rng.gen::<f64>());

            if (center - scene_focus).length() > 0.9 {
                let sphere_material : Material;

                if choose_mat < 0.8 {
                    //Diffuse
                    let albedo : Vec3 = Color3::random_vec() * Color3::random_vec();
                    sphere_material = Material::Lambertian { albedo: albedo };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }

                else if choose_mat < 0.95 {
                    //Metal 
                    let albedo : Vec3 = Color3::random_vec_range(0.5, 1.0);
                    let fuzz : f64 = rng.gen_range(0.0..0.5);
                    sphere_material = Material::Metal { albedo: albedo, fuzz: fuzz };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }

                else {
                    //Glass
                    sphere_material = Material::Diaelectric { index_of_refraction: 1.5 };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 : Material = Material::Diaelectric { index_of_refraction: 1.5 };
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 : Material = Material::Lambertian { albedo: Color3::new(0.4, 0.2, 0.1) };
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, -1.0, 0.0), 1.0, material2)));

    let material3 : Material = Material::Metal { albedo: Color3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    
    world_box = Box::new(world); //Last thing you should do!
    camera.render(&mut parameters, &mut world_box);    
}
