pub struct Tile {
    has_food: bool,
}

impl Tile {
    pub fn new(has_food: bool) -> Tile {
        Tile {
            has_food: has_food,
        }
    }

    pub fn has_food(&self) -> bool {
        self.has_food
    }

    pub fn set_food(&mut self, food: bool) {
        self.has_food = food;
    }
}
