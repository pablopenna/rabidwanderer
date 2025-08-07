use godot::prelude::*;
use godot::classes::*;

use crate::board::constants::BOARD_BACKGROUND_HEIGHT;
use crate::board::constants::BOARD_BACKGROUND_HEIGHT_OFFSET;
use crate::board::constants::BOARD_BACKGROUND_WIDTH;
use crate::board::constants::BOARD_BACKGROUND_WIDTH_OFFSET;
use crate::board::utils;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub(crate) struct BoardBackground {
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
    for base_x in 0..BOARD_BACKGROUND_WIDTH {
        for base_y in 0..BOARD_BACKGROUND_HEIGHT {
            let x: i32 = base_x as i32 + BOARD_BACKGROUND_WIDTH_OFFSET as i32;
            let y: i32 = base_y as i32 + BOARD_BACKGROUND_HEIGHT_OFFSET as i32;
            board.base_mut().set_cell_ex(Vector2i::from_tuple((x, y)))
                .source_id(0)
                .atlas_coords(Vector2i::from_tuple((0,0)))
                .alternative_tile(0)
                .done();
        }
    }
}
