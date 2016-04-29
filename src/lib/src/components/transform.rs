pub struct Transform {
    location: (f64, f64),
}

impl Transform {
    pub fn new_default() -> Transform {
        Transform::new((0.0, 0.0))
    }

    pub fn new(location: (f64, f64)) -> Transform {
        Transform {
            location: location,
        }
    }

    pub fn get_location(&self) -> (f64, f64) {
        self.location
    }
}
