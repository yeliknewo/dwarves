use lib_dwarves::*;

pub struct DwarvesEntity {
    id: Id,
    renderable: Option<Renderable>,
    transform: Option<Transform>,
}

impl_entity!(DwarvesEntity, id, renderable, transform);
impl_component_with_entity!(DwarvesEntity, renderable, Renderable, set_option_renderable, set_renderable, with_renderable);
impl_component_with_entity!(DwarvesEntity, transform, Transform, set_option_transform, set_transform, with_transform);

impl DwarvesEntity {
    pub fn new(id: Id) -> DwarvesEntity {
        DwarvesEntity {
            id: id,
            renderable: None,
            transform: None,
        }
    }
}
