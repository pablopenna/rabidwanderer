// use core::f32;
use godot::classes::*;
use godot::obj::NewAlloc;
use godot::prelude::*;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::board::background::BoardBackground;
use crate::board::data::generate_empty_board_data;
use crate::board::data::DataTile;
use crate::board::coordinate::coordinate_to_index;
use crate::board::coordinate::godot_vector_to_vector2d;
use crate::board::utils::verify_tile_set_exists;
use crate::board::constants::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub(crate) struct Board {
    random_generator: ThreadRng,
    data: [DataTile<'static>; BOARD_SIZE],
    background: Gd<BoardBackground>,
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for Board {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            random_generator: rand::rng(),
            data: generate_empty_board_data(),
            background: BoardBackground::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        verify_tile_set_exists(self.base().to_godot());
        Board::populate_board(self);

        // background node setup
        let tile_set = self.base().get_tile_set().unwrap();
        self.background.set_tile_set(&tile_set);
        
        self.base_mut().set_z_as_relative(true);
        self.base_mut().set_z_index(-1);
        self.background.set_z_as_relative(true);
        self.background.set_z_index(-1);
        
        let background_node = self.background.clone().upcast::<Node>();
        self.base_mut().add_child(&background_node);

        let gd_self = self.to_gd();
        self.signals().board_setted_up().emit(&gd_self);
    }

    fn physics_process(&mut self, _delta: f64) {}

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            godot_print!("Okay!!");
            let gd_self = self.to_gd();
            self.signals().board_setted_up().emit(&gd_self);
        }
    }
}

#[godot_api] // https://godot-rust.github.io/book/register/functions.html#user-defined-functions
impl Board {
    #[signal]
    pub(crate) fn board_setted_up(board: Gd<Board>);

    fn populate_board(&mut self) {
        let blocks_to_place: u8 = 20;
        let mut blocks_placed: u8 = 0;

        while blocks_placed < blocks_to_place {
            let x = self.random_generator.random_range(0..BOARD_WIDTH) as i32;
            let y = self.random_generator.random_range(0..BOARD_HEIGHT) as i32;
            let coord = Vector2i::from_tuple((x, y));
            if self.is_cell_empty(coord) {
                self.add_four_way_tile(coord);
                blocks_placed += 1;
            }
        }
    }

    fn is_cell_empty(&self, coordinates: Vector2i) -> bool {
        let cell = self.base().get_cell_tile_data(coordinates);
        return cell.is_none();
    }

    pub fn add_four_way_tile(&mut self, coordinates: Vector2i) {
        self.add_draw_tile(coordinates, FOUR_WAY_DRAW_CELL);

        let data_tile = &mut self.data[coordinate_to_index(godot_vector_to_vector2d(coordinates))];
        data_tile.make_traversable();
    }

    fn add_draw_tile(&mut self, coordinates: Vector2i, tile: DrawTile) {
        self
            .base_mut()
            .set_cell_ex(coordinates)
            .source_id(tile.source_id)
            .atlas_coords(tile.atlas_coords)
            .alternative_tile(tile.alternative_tile)
            .done();
    }

    pub(crate) fn get_data(&mut self) -> &[DataTile<'static>; BOARD_SIZE] {
        &self.data
    }
}

struct DrawTile {
    source_id: i32,
    atlas_coords: Vector2i,
    alternative_tile: i32,
}

const FOUR_WAY_DRAW_CELL: DrawTile = DrawTile {
    // coordinates: Vector2i::from_tuple((0, 0)),
    source_id: 0,
    alternative_tile: 0,
    atlas_coords: Vector2i::from_tuple((1, 0)),
};
