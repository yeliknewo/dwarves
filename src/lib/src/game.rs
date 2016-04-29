use std::sync::{Arc};
use scoped_threadpool::{Pool};
use piston_window::{PistonWindow, clear,};
use num_cpus;

use world::{World};
use entity::{Entity};

pub struct Game<T: Entity> {
    world: Arc<World<T>>,
    thread_pool: Pool,
}

impl<T: Entity> Game<T> {
    pub fn new() -> Game<T> {
        Game {
            world: Arc::new(World::new()),
            thread_pool: Pool::new(num_cpus::get() as u32),
        }
    }

    pub fn get_world(&self) -> Arc<World<T>> {
        self.world.clone()
    }

    pub fn get_mut_world(&mut self) -> &mut World<T> {
        if let Some(world) = Arc::get_mut(&mut self.world) {
            return world;
        }else{
            panic!("Get mut World was None");
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let world = self.get_world();
                let entities = world.get_entities();
                for entry in entities.iter() {
                    if let Some(renderable) = entry.1.get_renderable() {
                        renderable.draw_2d(c, g);
                    }
                }
            });
            if let Some(e) = e.update_args() {
                
            }
        }
    }
}
