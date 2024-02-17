use std::ops;
use std::fmt;
use std::fs;
use std::io::Write;
use rand::Rng;
use super::interval::Interval;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub vec : [f64; 3] 
}

pub type Color3 = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x : f64, y : f64, z : f64) -> Self {
        return Self {vec : [x,y,z]};
    }

    pub fn random_vec() -> Vec3 {
        let mut rng : rand::rngs::ThreadRng = rand::thread_rng();
        return Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
    }

    pub fn random_vec_range(min : f64, max : f64) -> Vec3 {
        let mut rng : rand::rngs::ThreadRng = rand::thread_rng();
        return Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p : Vec3 = Vec3::random_vec_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::random_in_unit_sphere().unit_vector();
    }

    //Generate a random vector and check if it is in the same hemisphere as the normal on a sphere.
    pub fn random_on_hemisphere(normal : &Vec3) -> Vec3 {
        let on_unit_sphere : Vec3 = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            //In the same hemisphere as the normal
            return on_unit_sphere;
        }

        return -1.0 * on_unit_sphere;
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng : rand::rngs::ThreadRng = rand::thread_rng();
        loop {
            let p : Vec3 = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0
            );

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f64{
        return self.vec[0];
    }

    pub fn y(&self) -> f64{
        return self.vec[1];
    }

    pub fn z(&self) -> f64 {
        return self.vec[2];
    }

    pub fn length_squared(&self) -> f64 {
        return self[0] * self[0] + self[1] * self[1] + self[2] * self[2];
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn magnitude(&self) -> f64 {
        return self.length();
    }

    pub fn dot(&self, other : &Vec3) -> f64 {
        return self[0] * other[0] + self[1] * other[1] + self[2] * other[2];
    }

    pub fn negate(&self) -> Vec3 {
        Vec3 {
            vec : [
                self[0] * -1.0,
                self[1] * -1.0,
                self[2] * -1.0
            ]
        }
    }

    pub fn cross(&self, other : &Vec3) -> Vec3 {
        return Vec3 {
            vec : [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0]   
            ]
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        return Vec3 {
            vec : [
                self[0] / length,
                self[1] / length,
                self[2] / length
            ]
        };
    }

    pub fn near_zero(&self) -> bool {
        static EPSILON : f64 = 1e-8;
        return (self[0].abs() < EPSILON) && (self[1].abs() < EPSILON) && (self[2].abs() < EPSILON);
    }

    pub fn reflect(&self, normal : &Vec3) -> Vec3 {
        return *self - 2.0*Vec3::dot(self, normal)*(*normal);
    }

    pub fn refract(uv : &Vec3, normal : &Vec3, refractive_ratio : f64) -> Vec3 {
        let cos_theta : f64 = Vec3::dot(&uv.negate(), normal).min(1.0);
        let r_out_perp : Vec3 = refractive_ratio * (cos_theta*(*normal) + *uv);
        let r_out_parallel : Vec3 = (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt()) * (*normal);
        return r_out_perp + r_out_parallel;
    }
}


impl Color3 {
    fn linear_to_gamma(linear_component : f64) -> f64 {
        return linear_component.sqrt();
    }

    //Should only be used by color3 variables, can still be used by vec3's though.
    pub fn write_color(&self, f : &mut fs::File, samples_per_pixel : u32) {
        static INTENSITY : Interval = Interval::new(0.0, 0.999);
        
        let mut r : f64 = self[0];
        let mut g : f64 = self[1];
        let mut b : f64 = self[2];

        //Divide the color by the number of samples
        let scale : f64 = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;

        //Gamma correction
        r = Color3::linear_to_gamma(r);
        g = Color3::linear_to_gamma(g);
        b = Color3::linear_to_gamma(b);
        
        writeln!(f, "{} {} {}", (256.0 * INTENSITY.clamp(r)) as u32, (256.0 * INTENSITY.clamp(g)) as u32, (256.0* INTENSITY.clamp(b)) as u32).expect("Failed to write to file");
    }
}

//Operator overloads and default constructors
impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3{
            vec : [0.0,0.0,0.0]
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       return write!(f, "[{}, {}, {}]", self[0], self[1], self[2]) 
    }
}


impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.vec[index];
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.vec[index];
    }
}


impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    
    fn add(self, rhs : Vec3) -> Self::Output {
        return Vec3 {
            vec: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2]
            ]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            vec : [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2]
            ]
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    
    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            vec : [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2]
            ]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            vec : [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2]
            ]
        };
    }
}

//Scalar multiplication, need to do both left and right sides.
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3 {
            vec : [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs
            ]
        };
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            vec : [
                self * rhs[0],
                self * rhs[1],
                self * rhs[2]
            ]
        };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            vec : [
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2]
            ]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            vec : [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs
            ]
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        return self * (1.0/rhs);
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        return rhs * (1.0/self);
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs;
    }
}

