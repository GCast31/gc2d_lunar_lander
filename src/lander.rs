
pub struct Lander {
    pub x: f32,
    pub y: f32,
    pub angle: f64,
    pub vx: f32,
    pub vy: f32,
    pub speed: f32,
    pub image: String,

    pub engine_enable: bool,
    pub image_engine: String,
    pub engine_x: f32,
    pub engine_y: f32,
}

impl Default for Lander {
    fn default() -> Self {
        Lander { 
            x: 0., 
            y: 0., 
            angle: 0., 
            vx: 0., 
            vy: 0., 
            speed: 0.,
            image: String::new(),
            engine_enable: false,
            image_engine: String::new(),
            engine_x: 0.,
            engine_y: 0.,
        }
    }
}