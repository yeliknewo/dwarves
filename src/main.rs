#[macro_use]
extern crate lib_dwarves;

use lib_dwarves::*;

mod dwarves_entity;
mod utils;
mod prefabs;
mod components;

use prefabs::{new_overseer, new_tile, TileType};
use utils::{DEntity};

fn main() {
    let title = "Dwarves";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 640])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });

    window.set_ups(60);

    let mut game = Game::<DEntity>::new();

    let mut manager = IdManager::new_default();
    {
        let mut world = game.get_mut_world();
        let overseer_id = {
            let overseer = new_overseer(&mut manager, &mut world);
            let id = overseer.get_id();
            world.add_entity(overseer);
            id
        };
        {
            for y in 0..5 {
                for x in 0..5 {
                    let tile = new_tile(&mut manager, x, y, TileType::Grass);
                    world.add_entity(tile);
                }
            }
        }
    }

    game.run(&mut manager, &mut window);
}
