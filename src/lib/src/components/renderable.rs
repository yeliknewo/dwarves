use piston_window::{Context, G2d, rectangle, Transformed};

use components::{Transform};

pub struct Renderable {
    color: [f32; 4],
    size: [f64; 4],
    location: (f64, f64),
    layer: u8,
}

impl Renderable {
    pub fn new(color: [f32;4], size: [f64; 4], location: (f64, f64), layer: u8) -> Renderable {
        Renderable {
            color: color,
            size: size,
            location: location,
            layer: layer,
        }
    }

    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        rectangle(self.color, self.size, c.transform.trans(self.location.0, self.location.1), g);
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn update(&mut self, transform: &Transform) {
        self.location = transform.get_location();
    }
}
