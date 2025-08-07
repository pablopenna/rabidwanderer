use godot::classes::*;
use godot::prelude::*;

use crate::board::graphics::draw_tile_board_background::BoardBackground;
use crate::board::graphics::utils::verify_tile_set_exists;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub(crate) struct DrawTileBoard {
    background: Gd<BoardBackground>,
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for DrawTileBoard {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            background: BoardBackground::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        verify_tile_set_exists(self.base().to_godot());

        // background node setup
        let tile_set = self.base().get_tile_set().unwrap();
        self.background.set_tile_set(&tile_set);
        
        self.base_mut().set_z_as_relative(true);
        self.base_mut().set_z_index(0);
        self.background.set_z_as_relative(true);
        self.background.set_z_index(-1);
        
        let background_node = self.background.clone().upcast::<Node>();
        self.base_mut().add_child(&background_node);
    }
}

