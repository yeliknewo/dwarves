use std::sync::{Arc};
use lib_dwarves::*;

use super::Job;
use utils::*;

pub struct JobWalk {
    dwarf: Id,
    path: Vec<Id>,
}

impl JobWalk {
    pub fn new(start: Id, end: Id, )
}

impl Job for JobWalk {
    fn tick(&self, dt: f64, world: Arc<DWorld>) {

    }

    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut DWorld) {

    }
}

impl_component!(JobWalk, true, true);
