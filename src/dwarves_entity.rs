use std::sync::{Arc};

use lib_dwarves::*;

use utils::*;
use components::*;

pub struct DwarvesEntity {
    id: Id,
    renderable: Option<Box<Renderable>>,
    transform: Option<Box<Transform>>,
    tile_map: Option<Box<TileMap>>,
    coords: Option<Box<Coords>>,
    container: Option<Box<Container>>,
    name: Option<Box<Name>>,
    dwarf: Option<Box<Dwarf>>,
}

impl_component_with_entity!(DwarvesEntity, renderable, Renderable, set_option_renderable, set_renderable, with_renderable, get_renderable, get_mut_renderable, take_renderable);
impl_component_with_entity!(DwarvesEntity, transform, Transform, set_option_transform, set_transform, with_transform, get_transform, get_mut_transform, take_transform);
impl_component_with_entity!(DwarvesEntity, tile_map, TileMap, set_option_tile_map, set_tile_map, with_tile_map, get_tile_map, get_mut_tile_map, take_tile_map);
impl_component_with_entity!(DwarvesEntity, container, Container, set_option_container, set_container, with_container, get_container, get_mut_container, take_container);
impl_component_with_entity!(DwarvesEntity, coords, Coords, set_option_coords, set_coords, with_coords, get_coords, get_mut_coords, take_coords);
impl_component_with_entity!(DwarvesEntity, name, Name, set_option_name, set_name, with_name, get_name, get_mut_name, take_name);
impl_component_with_entity!(DwarvesEntity, dwarf, Dwarf, set_option_dwarf, set_dwarf, with_dwarf, get_dwarf, get_mut_dwarf, take_dwarf);

impl DwarvesEntity {
    pub fn new(id: Id) -> DwarvesEntity {
        DwarvesEntity {
            id: id,
            renderable: None,
            transform: None,
            tile_map: None,
            coords: None,
            container: None,
            name: None,
            dwarf: None,
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
        self.dwarf.is_some()
    }

    fn is_tick_mut(&self) -> bool {
        self.dwarf.is_some()
    }
}
