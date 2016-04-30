use lib_dwarves::*;

use utils::{DEntity, CoordSize};

pub fn new_tile(manager: &mut IdManager, x: CoordSize, y: CoordSize) -> DEntity {
    let id = Id::new(manager);
    DEntity::new(id)
    .with_container(
        Container::new()
    )
    .with_coords(
        (x, y)
    )
    .with_renderable(
        Renderable::new(
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 16.0, 16.0],
            (x as f64 * 16.0, y as f64 * 16.0),
        )
    )
}
