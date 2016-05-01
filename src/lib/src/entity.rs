use std::sync::{Arc};

use id::{Id, IdManager};
use components::{Transform, Renderable};
use world::{World};

pub trait Entity<T: Entity<T>> : Send + Sync {
    fn get_id(&self) -> Id;
    fn get_renderable(&self) -> Option<&Box<Renderable>>;
    fn get_transform(&self) -> Option<&Box<Transform>>;
    fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>>;
    fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>>;
    fn tick(&self, dt: f64, world: Arc<World<T>>);
    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut World<T>);
    fn is_tick(&self) -> bool;
    fn is_tick_mut(&self) -> bool;
}

#[macro_export]
macro_rules! impl_component_with_entity {
    ($t: ident, $p: ident, $c: ident, $so: ident, $s: ident, $w: ident, $g: ident, $m: ident, $ta: ident) => (
        impl $t {
            #[inline]
            pub fn $so (&mut self, $p: Option<Box<$c>>) {
                self.$p = $p;
            }

            #[inline]
            pub fn $s (&mut self, $p: $c) {
                self.$so(Some(Box::new($p)));
            }

            #[inline]
            pub fn $w (mut self, $p: $c) -> $t {
                self.$s($p);
                self
            }

            #[inline]
            pub fn $g (&self) -> Option<&Box<$c>> {
                self.$p.as_ref()
            }

            #[inline]
            pub fn $m (&mut self) -> Option<&mut Box<$c>> {
                self.$p.as_mut()
            }

            #[inline]
            pub fn $ta (&mut self) -> Option<Box<$c>> {
                self.$p.take()
            }
        }
    )
}

#[macro_export]
macro_rules! impl_entity {
    ($t: ident, $id: ident, $r: ident, $tr: ident) => (
        #[inline]
        fn get_id(&self) -> Id {
            self.$id.clone()
        }

        #[inline]
        fn get_renderable(&self) -> Option<&Box<Renderable>> {
            self.$r.as_ref()
        }

        #[inline]
        fn get_transform(&self) -> Option<&Box<Transform>> {
            self.$tr.as_ref()
        }

        #[inline]
        fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>> {
            self.$r.as_mut()
        }

        #[inline]
        fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>> {
            self.$tr.as_mut()
        }
    )
}
