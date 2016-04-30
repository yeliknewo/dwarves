use lib_dwarves::*;

use utils::{DEntity, CoordSize};

#[inline]
pub fn new_tile(manager: &mut IdManager, x: CoordSize, y: CoordSize, tile_type: TileType) -> DEntity {
    match tile_type {
        TileType::Grass => new_grass_tile(manager, x, y),
        TileType::Stone => new_stone_tile(manager, x, y),
        TileType::Water => new_water_tile(manager, x, y),
    }
}

fn new_grass_tile(manager: &mut IdManager, x: CoordSize, y: CoordSize) -> DEntity {
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

fn new_stone_tile(manager: &mut IdManager, x: CoordSize, y: CoordSize) -> DEntity {
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
            [0.2, 0.2, 0.2, 1.0],
            [0.0, 0.0, 16.0, 16.0],
            (x as f64 * 16.0, y as f64 * 16.0),
        )
    )
}

fn new_water_tile(manager: &mut IdManager, x: CoordSize, y: CoordSize) -> DEntity {
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
            [0.0, 0.0, 1.0, 1.0],
            [0.0, 0.0, 16.0, 16.0],
            (x as f64 * 16.0, y as f64 * 16.0),
        )
    )
}

pub enum TileType {
    Grass,
    Stone,
    Water,
}
