use std::collections::{HashMap};
// use std::collections::hash_state::{DefaultState};
// use fnv::{FnvHasher};

use entity::{Entity};
use id::{Id};

pub struct World<T: Entity> {
    // entities: HashMap<Id, T, DefaultState<FnvHasher>>,
    entities: HashMap<Id, T>,
}

impl<T: Entity> World<T> {
    pub fn new() -> World<T> {
        World {
            entities: HashMap::new(),
        }
    }

    pub fn get_entity_by_id(&self, id: Id) -> Option<&T> {
        self.entities.get(&id)
    }

    pub fn get_mut_entity_by_id(&mut self, id: Id) -> Option<&mut T> {
        self.entities.get_mut(&id)
    }

    pub fn add_entity(&mut self, entity: T) {
        self.entities.insert(entity.get_id(), entity);
    }

    pub fn take_entity(&mut self, id: Id) -> Option<T> {
        self.entities.remove(&id)
    }

    pub fn get_entities(&self) -> &HashMap<Id, T> {
        &self.entities
    }

    pub fn get_mut_entities(&mut self) -> &mut HashMap<Id, T> {
        &mut self.entities
    }
}
