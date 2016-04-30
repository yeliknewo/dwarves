use std::sync::{Arc};

use id::{Id, IdManager};
use components::{Transform, Renderable};
use world::{World};

pub trait Entity<T: Entity<T>> : Send + Sync {
    fn get_id(&self) -> Id;
    fn get_renderable(&self) -> Option<&Renderable>;
    fn get_transform(&self) -> Option<&Transform>;
    fn get_mut_renderable(&mut self) -> Option<&mut Renderable>;
    fn get_mut_transform(&mut self) -> Option<&mut Transform>;
    fn tick(&self, dt: f64, world: Arc<World<T>>);
    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut World<T>);
    fn is_tick(&self) -> bool;
    fn is_tick_mut(&self) -> bool;
}

#[macro_export]
macro_rules! impl_component_with_entity {
    ($t: ident, $p: ident, $c: ident, $so: ident, $s: ident, $w: ident) => (
        impl $t {
            #[inline]
            pub fn $so (&mut self, $p: Option<$c>) {
                self.$p = $p;
            }

            #[inline]
            pub fn $s (&mut self, $p: $c) {
                self.$so(Some($p));
            }

            #[inline]
            pub fn $w (mut self, $p: $c) -> $t {
                self.$s($p);
                self
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
        fn get_renderable(&self) -> Option<&Renderable> {
            self.$r.as_ref()
        }

        #[inline]
        fn get_transform(&self) -> Option<&Transform> {
            self.$tr.as_ref()
        }

        #[inline]
        fn get_mut_renderable(&mut self) -> Option<&mut Renderable> {
            self.$r.as_mut()
        }

        #[inline]
        fn get_mut_transform(&mut self) -> Option<&mut Transform> {
            self.$tr.as_mut()
        }
    )
}
