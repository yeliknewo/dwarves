use std::sync::{Arc};

use lib_dwarves::*;

use utils::{DWorld, TileMap, Coords};

pub struct DwarvesEntity {
    id: Id,
    renderable: Option<Renderable>,
    transform: Option<Transform>,
    tile_map: Option<TileMap>,
    coords: Option<Coords>,
    container: Option<Container>,
}

impl_component_with_entity!(DwarvesEntity, renderable, Renderable, set_option_renderable, set_renderable, with_renderable);
impl_component_with_entity!(DwarvesEntity, transform, Transform, set_option_transform, set_transform, with_transform);
impl_component_with_entity!(DwarvesEntity, tile_map, TileMap, set_option_tile_map, set_tile_map, with_tile_map);
impl_component_with_entity!(DwarvesEntity, container, Container, set_option_container, set_container, with_container);
impl_component_with_entity!(DwarvesEntity, coords, Coords, set_option_coords, set_coords, with_coords);

impl DwarvesEntity {
    pub fn new(id: Id) -> DwarvesEntity {
        DwarvesEntity {
            id: id,
            renderable: None,
            transform: None,
            tile_map: None,
            coords: None,
            container: None,
        }
    }
}

impl Entity<DwarvesEntity> for DwarvesEntity {
    impl_entity!(DwarvesEntity, id, renderable, transform);

    fn tick(&self, dt: f64, world: Arc<DWorld>) {

    }

    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut DWorld) {

    }

    fn is_tick(&self) -> bool {
        false
    }

    fn is_tick_mut(&self) -> bool {
        false
    }
}
