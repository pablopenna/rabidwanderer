use godot::builtin::Vector2i;

pub(crate) struct DrawTile {
    source_id: i32,
    atlas_coords: Vector2i,
    alternative_tile: i32,
}

impl DrawTile {
    pub(crate) fn get_source_id(&self) -> i32 {
        self.source_id
    }
    pub(crate) fn get_atlas_coords(&self) -> Vector2i {
        self.atlas_coords
    }
    pub(crate) fn get_alternative_tile(&self) -> i32 {
        self.alternative_tile
    }
}

pub(crate) const FOUR_WAY_DRAW_TILE: DrawTile = DrawTile {
    source_id: 0,
    alternative_tile: 0,
    atlas_coords: Vector2i::from_tuple((1, 0)),
};

pub(crate) const BASE_DRAW_TILE: DrawTile = DrawTile {
    source_id: 0,
    alternative_tile: 0,
    atlas_coords: Vector2i::from_tuple((0, 0)),
};