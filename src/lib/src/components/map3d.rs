use std::collections::{HashMap};
use std::hash::{Hash};

use id::{Id};

pub struct Map3d<T: Hash + Eq + Copy> {
    tiles: HashMap<(T, T, T), Id>,
}

impl<T: Hash + Eq + Copy> Map3d<T>{
    pub fn new() -> Map3d<T> {
        Map3d {
            tiles: HashMap::new(),
        }
    }

    pub fn insert(&mut self, x: T, y: T, z: T, id: Id) {
        self.tiles.insert((x, y, z), id);
    }

    pub fn get(&self, x: T, y: T, z: T) -> Option<Id> {
        match self.tiles.get(&(x, y, z)) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
}
