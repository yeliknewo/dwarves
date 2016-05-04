pub struct Tile {
    walkable: bool,
}

impl_component!(Tile, false, false);

impl Tile {
    pub fn new(walkable: bool) -> Tile {
        Tile {
            walkable: walkable,
        }
    }

    pub fn is_walkable(&self) -> bool {
        self.walkable
    }
}
