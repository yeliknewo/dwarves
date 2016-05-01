#[macro_use]
extern crate lib_dwarves;
extern crate rand;

use lib_dwarves::*;

mod dwarves_entity;
mod utils;
mod prefabs;
mod components;

use prefabs::*;
use utils::*;

use rand::Rng;

pub const TILE_TYPE_ARRAY: [TileType; 3] = [TileType::Grass, TileType::Stone, TileType::Water];

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

    let mut rng = rand::thread_rng();

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
            for y in 0..40 {
                for x in 0..40 {
                    let tile = new_tile(&mut manager, x, y, rng.choose(&TILE_TYPE_ARRAY).expect("Tile type array was empty").clone());
                    {
                        let mut overseer = world.get_mut_entity_by_id(overseer_id.clone()).expect("Overseer was none");
                        let mut tile_map = overseer.get_mut_tile_map().expect("Overseer had no tile map");
                        tile_map.insert_split(x, y, tile.get_id());
                    }
                    world.add_entity(tile);
                }
            }
        }
        let dwarf_count = 20;
        for i in 0..dwarf_count {
            let tile_id = {
                let overseer = world.get_entity_by_id(overseer_id.clone()).expect("Overseer was none");
                let tile_map = overseer.get_tile_map().expect("Tile map was none");
                tile_map.get_split(i, i).expect("Tile map was empty at 0, 0")
            };
            {
                let dwarf = new_dwarf(&mut manager, tile_id, world);
                world.add_entity(dwarf);
            }
        }
    }

    game.run(&mut manager, &mut window);
}
