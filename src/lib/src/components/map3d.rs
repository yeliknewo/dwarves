use std::collections::{HashMap};
use std::hash::{Hash};

use id::{Id};

pub struct Map3d<T: Hash + Eq + Copy> {
    tiles: HashMap<(T, T, T), Id>,
}

impl<T: Hash + Eq + Copy> Map3d<T>{
    #[inline]
    pub fn new() -> Map3d<T> {
        Map3d {
            tiles: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, coords: (T, T, T), id: Id) {
        self.tiles.insert(coords, id);
    }

    #[inline]
    pub fn insert_split(&mut self, x: T, y: T, z: T, id: Id) {
        self.insert((x, y, z), id);
    }

    #[inline]
    pub fn get(&self, coords: (T, T, T)) -> Option<Id> {
        match self.tiles.get(&coords) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn get_split(&self, x: T, y: T, z: T) -> Option<Id> {
        self.get((x,y,z))
    }
}
