use std::collections::{HashMap};
use std::hash::{Hash};

use id::{Id};

pub struct Map2d<T: Hash + Eq + Copy> {
    tiles: HashMap<(T, T), Id>,
}

impl<T: Hash + Eq + Copy> Map2d<T>{
    pub fn new() -> Map2d<T> {
        Map2d {
            tiles: HashMap::new(),
        }
    }

    pub fn insert(&mut self, x: T, y: T, id: Id) {
        self.tiles.insert((x, y), id);
    }

    pub fn get(&self, x: T, y: T) -> Option<Id> {
        match self.tiles.get(&(x, y)) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
}
