pub struct Interval {
    pub min : f64,
    pub max : f64
}

pub const empty : Interval = Interval {
    min : f64::INFINITY,
    max : f64::NEG_INFINITY
};

pub const universe : Interval = Interval {
    min : f64::NEG_INFINITY,
    max : f64::INFINITY
};

impl Interval {
    pub const fn new(_min : f64, _max : f64) -> Interval {
        return Interval {
            min : _min,
            max : _max
        }
    }

    //If the value is at the boundary or within the interval
    pub fn contains(&self, x : f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    //If the value is within the interval only.
    pub fn surrounds(&self, x : f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x : f64) -> f64 {
        if(x < self.min){
            return self.min;
        }
        if(x > self.max){
            return self.max;
        }
        
        return x;
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {min : f64::INFINITY, max : f64::NEG_INFINITY}
    }
}



