use lib_dwarves::*;

use utils::*;
use components::*;

pub fn new_dwarf(manager: &mut IdManager, tile_id: Id, world: &DWorld) -> DEntity {
    let id = Id::new(manager);
    DEntity::new(id)
    .with_renderable(
        Renderable::new(
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, TILE_SIZE, TILE_SIZE],
            (0.0, 0.0),
            DWARF_LAYER,
        )
    )
    .with_transform(
        Transform::new(
            world.get_entity_by_id(tile_id.clone()).expect("Tile id was not an entity").get_transform().expect("Tile Entity had no Transform").get_location()
        )
    )
    .with_dwarf(
        Dwarf::new(tile_id)
    )
}
