pub struct Tile {
    food: bool,
    walkable: bool,
}

impl Tile {
    pub fn new(food: bool, walkable: bool) -> Tile {
        Tile {
            food: food,
            walkable: walkable,
        }
    }

    pub fn is_walkable(&self) -> bool {
        self.walkable
    }

    pub fn has_food(&self) -> bool {
        self.food
    }

    pub fn set_food(&mut self, food: bool) {
        self.food = food;
    }
}
