#[macro_use]
extern crate lib_dwarves;

use lib_dwarves::*;

mod dwarves_entity;
mod utils;
mod prefabs;
mod components;

use prefabs::*;
use utils::*;

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
                    {
                        let mut overseer = world.get_mut_entity_by_id(overseer_id.clone()).expect("Overseer was none");
                        let mut tile_map = overseer.get_mut_tile_map().expect("Overseer had no tile map");
                        tile_map.insert_split(x, y, tile.get_id());
                    }
                    world.add_entity(tile);
                }
            }
        }
        let tile_id = {
            let overseer = world.get_entity_by_id(overseer_id).expect("Overseer was none");
            let tile_map = overseer.get_tile_map().expect("Tile map was none");
            tile_map.get_split(1, 1).expect("Tile map was empty at 1, 1")
        };
        {
            let dwarf = new_dwarf(&mut manager, tile_id, world);
            world.add_entity(dwarf);
        }
    }

    game.run(&mut manager, &mut window);
}
