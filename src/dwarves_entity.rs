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
    tile: Option<Box<Tile>>,
    item: Option<Box<Item>>,
}

impl_component_with_entity!(DwarvesEntity, renderable, Renderable, set_option_renderable, set_renderable, with_renderable, get_renderable, get_mut_renderable, take_renderable, give_renderable);
impl_component_with_entity!(DwarvesEntity, transform, Transform, set_option_transform, set_transform, with_transform, get_transform, get_mut_transform, take_transform, give_transform);
impl_component_with_entity!(DwarvesEntity, tile_map, TileMap, set_option_tile_map, set_tile_map, with_tile_map, get_tile_map, get_mut_tile_map, take_tile_map, give_tile_map);
impl_component_with_entity!(DwarvesEntity, container, Container, set_option_container, set_container, with_container, get_container, get_mut_container, take_container, give_container);
impl_component_with_entity!(DwarvesEntity, coords, Coords, set_option_coords, set_coords, with_coords, get_coords, get_mut_coords, take_coords, give_coords);
impl_component_with_entity!(DwarvesEntity, name, Name, set_option_name, set_name, with_name, get_name, get_mut_name, take_name, give_name);
impl_component_with_entity!(DwarvesEntity, dwarf, Dwarf, set_option_dwarf, set_dwarf, with_dwarf, get_dwarf, get_mut_dwarf, take_dwarf, give_dwarf);
impl_component_with_entity!(DwarvesEntity, tile, Tile, set_option_tile, set_tile, with_tile, get_tile, get_mut_tile, take_tile, give_tile);
impl_component_with_entity!(DwarvesEntity, item, Item, set_option_item, set_item, with_item, get_item, get_mut_item, take_item, give_item);

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
            tile: None,
            item: None,
        }
    }
}

impl Entity<DwarvesEntity> for DwarvesEntity {
    impl_entity!(id, renderable, transform);

    fn tick(&self, dt: f64, world: Arc<DWorld>) {
        if let Some(ref dwarf) = self.dwarf {
            dwarf.tick(dt, world);
        }
    }

    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut DWorld) {
        if self.dwarf.is_some() {
            let mut transform = self.take_transform().expect("Dwarf didn't have transform");
            let mut renderable = self.take_renderable().expect("Dwarf didn't have renderable");
            let id = self.get_id().clone();
            self.get_mut_dwarf().expect("Dwarf wasn't a dwarf?").tick_mut(id, world, &mut transform, &mut renderable);
            self.give_transform(transform);
            self.give_renderable(renderable);
        }
    }

    fn is_tick(&self) -> bool {
        match self.dwarf.as_ref() {
            Some(dwarf) => dwarf.is_tick(),
            None => false,
        }
    }

    fn is_tick_mut(&self) -> bool {
        match self.dwarf.as_ref() {
            Some(dwarf) => dwarf.is_tick_mut(),
            None => false,
        }
    }
}
