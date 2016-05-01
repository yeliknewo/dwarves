use lib_dwarves::*;

use utils::*;
use dwarves_entity::*;

pub struct Dwarf {
    tile_id: Id,
}

impl Dwarf {
    pub fn new(tile_id: Id) -> Dwarf {
        Dwarf {
            tile_id: tile_id,
        }
    }

    pub fn tick(&self) {

    }

    pub fn tick_mut(&mut self, my_id: Id, world: &mut DWorld, transform: &mut Box<Transform>) {

    }

    fn travel(&mut self, my_id: Id, target: Id, world: &mut DWorld, transform: &mut Transform) {
        {
            let mut tile = world.get_mut_entity_by_id(self.tile_id.clone()).expect("Current Location was not an entity");
            tile.get_mut_container().expect("Tile wasn't a container").remove_id(my_id.clone());
        }
        let location = {
            let mut tile = world.get_mut_entity_by_id(target.clone()).expect("Target Location was not an entity");
            self.tile_id = target;
            tile.get_mut_container().expect("Target wasn't a container").add_id(my_id.clone());
            tile.get_coords().expect("Target wasn't a coords").clone()
        };
        transform.set_location((location.0 as f64 * 16.0, location.1 as f64 * 16.0));
    }
}
