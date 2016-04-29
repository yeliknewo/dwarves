#[macro_use]
extern crate lib_dwarves;

use lib_dwarves::*;

mod dwarves_entity;

use dwarves_entity::DwarvesEntity as DEntity;

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
        let id = Id::new(&mut manager);
        world.add_entity(
            DEntity::new(id)
            .with_renderable(
                Renderable::new(
                    [1.0, 0.0, 0.0, 1.0],
                    [0.0, 0.0, 16.0, 16.0],
                    (0.0, 0.0)
                )
            )
        );
    }

    game.run(&mut window);
}
