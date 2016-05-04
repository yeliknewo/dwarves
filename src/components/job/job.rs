use std::sync::{Arc};
use lib_dwarves::*;

use utils::*;

pub trait Job : Send + Sync + Component {
    fn tick(&self, dt: f64, world: Arc<DWorld>);
    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut DWorld);
}
