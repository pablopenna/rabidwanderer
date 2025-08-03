// use core::f32;

use godot::prelude::*;
use godot::classes::*;

const BOARD_WIDTH: u16 = 100;
const BOARD_HEIGHT: u16 = 50;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct Board {
    base: Base<TileMapLayer>
}

#[godot_api]
impl ITileMapLayer for Board {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        verify_tile_set_exists(self);

        add_board_base_tiles(self);
    }

    fn physics_process(&mut self, _delta: f64) {
       
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed( "ui_accept") {
            godot_print!("Okay!!");
        }
    }

}

fn verify_tile_set_exists(board: &Board) {
    if board.base().get_tile_set().is_none() {
        panic!("No tileset provided for the board!");
    }
}

fn add_board_base_tiles(board: &mut Board) {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            board.base_mut().set_cell_ex(Vector2i::from_tuple((x.into(), y.into())))
                .source_id(0)
                .atlas_coords(Vector2i::from_tuple((0,0)))
                .alternative_tile(0)
                .done();
        }
    }
}
