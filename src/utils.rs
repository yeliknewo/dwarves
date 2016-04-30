use dwarves_entity::{DwarvesEntity};
use lib_dwarves::{World, Map2d};

pub type DEntity = DwarvesEntity;
pub type DWorld = World<DEntity>;
pub type TileMap = Map2d<CoordSize>;
pub type Coords = (u32, u32);
pub type CoordSize = u32;
