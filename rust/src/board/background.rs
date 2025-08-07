use godot::prelude::*;
use godot::classes::*;

use crate::board::utils;

const BOARD_BACKGROUND_WIDTH: u16 = 100;
const BOARD_BACKGROUND_HEIGHT: u16 = 100;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct BoardBackground {
    base: Base<TileMapLayer>
}

#[godot_api]
impl ITileMapLayer for BoardBackground {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        utils::verify_tile_set_exists(self.base().to_godot());
        add_board_base_tiles(self);
    }

}

fn add_board_base_tiles(board: &mut BoardBackground) {
    for x in 0..BOARD_BACKGROUND_WIDTH {
        for y in 0..BOARD_BACKGROUND_HEIGHT {
            board.base_mut().set_cell_ex(Vector2i::from_tuple((x.into(), y.into())))
                .source_id(0)
                .atlas_coords(Vector2i::from_tuple((0,0)))
                .alternative_tile(0)
                .done();
        }
    }
}
