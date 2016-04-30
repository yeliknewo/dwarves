use lib_dwarves::*;

use utils::*;
use components::*;

pub fn new_dwarf(manager: &mut IdManager, tile_id: Id) -> DEntity {
    let id = Id::new(manager);
    DEntity::new(id)
    .with_renderable(
        Renderable::new(
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 16.0, 16.0],
            (0.0, 0.0)
        )
    )
    .with_dwarf(
        Dwarf::new(tile_id)
    )
}
