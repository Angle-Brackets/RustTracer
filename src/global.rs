pub struct Parameters {
    pub width : u32,
    pub height : u32,
    pub viewport_height: f64, 
    pub viewport_width: f64,
}

impl Parameters {
    pub fn new(width : u32, height : u32) -> Self {
        return Self {
            width : width, 
            height : height,
            viewport_height : 2.0,
            viewport_width : 2.0 * ((width as f64)/(height as f64))
        };
    }
}