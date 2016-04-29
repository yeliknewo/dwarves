use id::{Id};
use components::{Transform, Renderable};

pub trait Entity : Send + Sync {
    fn get_id(&self) -> Id;
    fn get_renderable(&self) -> Option<&Renderable>;
    fn get_transform(&self) -> Option<&Transform>;
    fn get_mut_renderable(&mut self) -> Option<&mut Renderable>;
    fn get_mut_transform(&mut self) -> Option<&mut Transform>;
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
        impl Entity for $t {
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
        }
    )
}
