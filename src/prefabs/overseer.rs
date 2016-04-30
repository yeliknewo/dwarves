use lib_dwarves::*;

use utils::{DEntity, TileMap};

pub fn new_overseer(manager: &mut IdManager) -> DEntity {
    let id = Id::new(manager);
    DEntity::new(id)
    .with_tile_map(
        TileMap::new()
    )
}
