use std::collections::{HashMap};
use std::hash::{Hash};

use id::{Id};

pub struct Map2d<T: Hash + Eq + Copy> {
    tiles: HashMap<(T, T), Id>,
}

impl<T: Hash + Eq + Copy> Map2d<T>{
    #[inline]
    pub fn new() -> Map2d<T> {
        Map2d {
            tiles: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, coords: (T, T), id: Id) {
        self.tiles.insert(coords, id);
    }

    #[inline]
    pub fn insert_split(&mut self, x: T, y: T, id: Id) {
        self.insert((x, y), id);
    }

    #[inline]
    pub fn get(&self, coords: (T, T)) -> Option<Id> {
        match self.tiles.get(&coords) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn get_split(&self, x: T, y: T) -> Option<Id> {
        self.get((x,y))
    }
}
