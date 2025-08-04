// use core::f32;
use godot::classes::*;
use godot::prelude::*;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::board::board_utils;

const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 10;

const TILE_SET_SOURCE_ID: i32 = 0;
const TILE_SET_ALTERNATIVE_TILE: i32 = 0;
const TILE_SET_FOURWAY_ATLAS_COORD: Vector2i = Vector2i::from_tuple((1, 0));

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct Board {
    random_generator: ThreadRng,
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for Board {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            random_generator: rand::rng(),
            base,
        }
    }

    fn ready(&mut self) {
        board_utils::verify_tile_set_exists(self.base().to_godot());
        populate_board(self);
    }

    fn physics_process(&mut self, _delta: f64) {}

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            godot_print!("Okay!!");
        }
    }
}

fn populate_board(board: &mut Board) {
    let blocks_to_place: u8 = 100;
    let mut blocks_placed: u8 = 0;

    while blocks_placed < blocks_to_place {
        let x = board.random_generator.random_range(0..BOARD_WIDTH);
        let y = board.random_generator.random_range(0..BOARD_HEIGHT);
        let coord = Vector2i::from_tuple((x, y));
        if is_cell_empty(board, coord) {
            add_four_way_cell(board, coord);
            blocks_placed += 1;
        }
    }
}

fn is_cell_empty(board: &mut Board, coordinates: Vector2i) -> bool {
    let cell = board.base().get_cell_tile_data(coordinates);
    return cell.is_none();
}

fn add_four_way_cell(board: &mut Board, coordinates: Vector2i) {
    board
        .base_mut()
        .set_cell_ex(coordinates)
        .source_id(TILE_SET_SOURCE_ID)
        .atlas_coords(TILE_SET_FOURWAY_ATLAS_COORD)
        .alternative_tile(TILE_SET_ALTERNATIVE_TILE)
        .done();
}
