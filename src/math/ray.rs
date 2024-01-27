use super::vec3::Vec3;

pub struct Ray {
    pub origin : Vec3,
    pub dir : Vec3
}

impl Ray {
    pub fn new(orig : Vec3, direction : Vec3) -> Self {
        Self {
            origin : orig,
            dir : direction
        }
    }

    pub fn at(&self, t : f64) -> Vec3 {
       return self.origin + t * self.dir;
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin : Vec3::default(),
            dir : Vec3::default()
        }
    }
}
