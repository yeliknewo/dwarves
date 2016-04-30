use lib_dwarves::*;

use utils::{DEntity, DWorld, TileMap, OVERSEER_NAME};

pub fn new_overseer(manager: &mut IdManager, world: &mut DWorld) -> DEntity {
    let id = Id::new(manager);
    DEntity::new(id.clone())
    .with_tile_map(
        TileMap::new()
    )
    .with_name(
        Name::new(OVERSEER_NAME, id, world)
    )
}
