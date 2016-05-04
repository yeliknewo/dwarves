pub struct Item {
    edible: bool,
    nutrition: f64,
}

impl_component!(Item, false, false);

impl Item {
    pub fn new() -> Item {
        Item {
            edible: false,
            nutrition: 0.0,
        }
    }

    pub fn new_food(nutrition: f64, edible: bool) -> Item {
        Item {
            edible: edible,
            nutrition: nutrition,
        }
    }

    pub fn is_edible(&self) -> bool {
        self.edible
    }

    pub fn get_nutrition(&self) -> f64 {
        self.nutrition
    }

    pub fn set_edible(&mut self, edible: bool) {
        self.edible = edible;
    }

    pub fn set_nutrition(&mut self, nutrition: f64) {
        self.nutrition = nutrition;
    }
}
