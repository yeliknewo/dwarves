use std::collections::{HashMap, HashSet};
// use std::collections::hash_state::{DefaultState};
// use fnv::{FnvHasher};

use entity::{Entity};
use id::{Id};

pub struct World<T: Entity<T>> {
    // entities: HashMap<Id, T, DefaultState<FnvHasher>>,
    entities: HashMap<Id, T>,
    tick_ids: HashSet<Id>,
    tick_mut_ids: Option<HashSet<Id>>,
    render_ids: HashSet<Id>,
}

impl<T: Entity<T>> World<T> {
    #[inline]
    pub fn new() -> World<T> {
        World {
            entities: HashMap::new(),
            tick_ids: HashSet::new(),
            tick_mut_ids: Some(HashSet::new()),
            render_ids: HashSet::new(),
        }
    }

    #[inline]
    pub fn get_entity_by_id(&self, id: Id) -> Option<&T> {
        self.entities.get(&id)
    }

    #[inline]
    pub fn get_mut_entity_by_id(&mut self, id: Id) -> Option<&mut T> {
        self.entities.get_mut(&id)
    }

    #[inline]
    pub fn add_entity(&mut self, entity: T) {
        let id = entity.get_id();
        self.entities.insert(id.clone(), entity);
        self.add_event_ids(id);
    }

    #[inline]
    pub fn take_entity_by_id(&mut self, id: Id) -> Option<T> {
        self.entities.remove(&id)
    }

    #[inline]
    pub fn give_entity(&mut self, entity: T) {
        self.entities.insert(entity.get_id(), entity);
    }

    #[inline]
    pub fn get_entities(&self) -> &HashMap<Id, T> {
        &self.entities
    }

    #[inline]
    pub fn get_mut_entities(&mut self) -> &mut HashMap<Id, T> {
        &mut self.entities
    }

    fn add_event_ids(&mut self, id: Id) {
        let mut tick = false;
        let mut tick_mut = false;
        let mut render = false;

        if let Some(entity) = self.get_entity_by_id(id.clone()) {
            tick = entity.is_tick();
            tick_mut = entity.is_tick_mut();
            render = entity.get_renderable().is_some();
        }

        if tick {
            self.tick_ids.insert(id.clone());
        }

        if tick_mut {
            if let Some(ref mut tick_mut_ids) = self.tick_mut_ids {
                tick_mut_ids.insert(id.clone());
            }
        }
        
        if render {
            self.render_ids.insert(id.clone());
        }
    }

    fn remove_event_ids(&mut self, id: Id) {
        self.tick_ids.remove(&id);
        if let Some(ref mut tick_mut_ids) = self.tick_mut_ids {
            tick_mut_ids.remove(&id);
        }
        self.render_ids.remove(&id);
    }

    #[inline]
    pub fn update_event_ids_by_id(&mut self, id: Id) {
        self.remove_event_ids(id.clone());
        self.add_event_ids(id);
    }

    #[inline]
    pub fn get_tick_ids(&self) -> &HashSet<Id> {
        &self.tick_ids
    }

    #[inline]
    pub fn get_render_ids(&self) -> &HashSet<Id> {
        &self.render_ids
    }

    #[inline]
    pub fn take_tick_mut_ids(&mut self) -> Option<HashSet<Id>> {
        self.tick_mut_ids.take()
    }

    #[inline]
    pub fn give_tick_mut_ids(&mut self, tick_mut_ids: HashSet<Id>) {
        self.tick_mut_ids = Some(tick_mut_ids);
    }
}
