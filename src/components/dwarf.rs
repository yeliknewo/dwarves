use std::sync::{Arc, RwLock};
use rand::Rng;
use rand;

use lib_dwarves::*;

use utils::*;

const MOVE_OPTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1)
];

pub struct Dwarf {
    tile_id: Id,
    hunger: f64,
    hunger_threshhold: f64,
}

impl_component!(Dwarf, false, false);

impl Dwarf {
    pub fn new(tile_id: Id) -> Dwarf {
        Dwarf {
            tile_id: tile_id,
            hunger: 2.0,
            hunger_threshhold: 1.0,
        }
    }
    //
    // pub fn tick(&self, dt: f64, world: Arc<DWorld>) {
    //     let mut changes = self.changes.write().expect("Poisoned Changes");
    //     if changes.new_hunger.is_some() {
    //         changes.new_hunger = Some((changes.new_hunger.expect("New hunger was none?") - dt).max(0.0));
    //     } else {
    //         changes.new_hunger = Some((self.hunger - dt).max(0.0));
    //     }
    //     if self.hunger < self.hunger_threshhold {
    //         for entity_id in world.get_entity_by_id(self.tile_id.clone()).expect("Current tile was none").get_container().expect("Tile had no container").get_ids().iter() {
    //             if let Some(item) = world.get_entity_by_id(entity_id.clone()).expect("Entity was none").get_item() {
    //                 if item.is_edible() {
    //                     changes.should_eat = true;
    //                     break;
    //                 }
    //             }
    //         }
    //         if world.get_entity_by_id(self.tile_id.clone()).expect("Current Tile was none").get_tile().expect("Tile had no tile").has_food() {
    //             changes.should_eat = true;
    //         } else {
    //             let coords = world.get_entity_by_id(self.tile_id.clone()).expect("Tile was none").get_coords().expect("Coords on tile was none").clone();
    //             let new_coords = {
    //                 let mut new_coords = coords;
    //                 let mut rng = rand::thread_rng();
    //                 let delta_coords = rng.choose(&MOVE_OPTIONS).expect("choose bullshit rng stuff").clone();
    //                 {
    //                     let temp_x = delta_coords.0 + new_coords.get_x() as i32;
    //                     if temp_x >= 0 {
    //                         new_coords.set_x(temp_x as u32);
    //                     }
    //                 }
    //                 {
    //                     let temp_y = delta_coords.1 + new_coords.get_y() as i32;
    //                     if temp_y >= 0 {
    //                         new_coords.set_y(temp_y as u32);
    //                     }
    //                 }
    //                 new_coords
    //             };
    //             {
    //                 if let Some(target_id) = world.get_entity_by_name(OVERSEER_NAME).expect("Overseer was none").get_tile_map().expect("Overseer had no tile map").get(&new_coords) {
    //                     if world.get_entity_by_id(target_id.clone()).expect("Target id was none").get_tile().expect("Target tile had no tile").is_walkable() {
    //                         changes.target = Some(target_id);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // pub fn tick_mut(&mut self, my_id: Id, world: &mut DWorld, transform: &mut Box<Transform>, renderable: &mut Box<Renderable>) {
    //     let target = {
    //         let mut changes = self.changes.write().expect("Poisoned Changes");
    //         if let Some(new_hunger) = changes.new_hunger.take() {
    //             self.hunger = new_hunger;
    //         }
    //         if changes.should_eat {
    //             changes.should_eat = false;
    //             let mut tile = world.get_mut_entity_by_id(self.tile_id.clone()).expect("My tile was none");
    //             tile.get_mut_tile().expect("Tile had no tile").set_food(false);
    //             tile.get_mut_renderable().expect("Tile had no renderable").set_color([0.2, 0.2, 0.2, 1.0]);
    //             self.hunger += 2.0;
    //         }
    //         changes.target.take()
    //     };
    //     if let Some(target) = target {
    //         self.travel(my_id, target, world, transform, renderable);
    //     }
    // }

    pub fn travel(&mut self, my_id: Id, target: Id, world: &mut DWorld, transform: &mut Transform, renderable: &mut Renderable) {
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
        transform.set_location((location.get_x() as f64 * 16.0, location.get_y() as f64 * 16.0));
        renderable.update(transform);
    }
}
