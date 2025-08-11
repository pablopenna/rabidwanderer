use godot::classes::*;
use godot::prelude::*;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::board::coordinate::BoardCoordinate;
use crate::board::data::data_tile::generate_empty_board_data;
use crate::board::data::data_tile::DataTile;
use crate::board::graphics::draw_tile_board::DrawTileBoard;
use crate::board::graphics::utils as DrawBoardUtils;
use crate::board::constants::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub(crate) struct Board {
    random_generator: ThreadRng,
    data: [DataTile; BOARD_SIZE],
    graphics: Gd<DrawTileBoard>,
    #[export]
    tile_set: OnEditor<Gd<TileSet>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Board {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            random_generator: rand::rng(),
            data: generate_empty_board_data(),
            graphics: DrawTileBoard::new_alloc(),
            // https://godot-rust.github.io/docs/gdext/master/godot/prelude/struct.OnEditor.html#example-user-defined-init
            tile_set: OnEditor::default(),
            base,
        }
    }

    fn ready(&mut self) {
        let tile_set = self.tile_set.to_godot();
        self.graphics.set_tile_set(&tile_set);
        
        self.populate_board();

        let graphics_node = self.graphics.clone().upcast::<Node>();
        self.base_mut().add_child(&graphics_node);

        self.signals().board_setted_up().emit();
    }

    fn physics_process(&mut self, _delta: f64) {}

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            godot_print!("Okay!!");
            self.signals().board_setted_up().emit();
        }
    }
}

#[godot_api] // https://godot-rust.github.io/book/register/functions.html#user-defined-functions
impl Board {
    #[signal]
    pub(crate) fn board_setted_up();

    fn populate_board(&mut self) {
        let blocks_to_place: u8 = 150;
        let mut blocks_placed: u8 = 0;
        let draw_tile_board = &mut self.graphics.clone().upcast::<TileMapLayer>();

        while blocks_placed < blocks_to_place {
            let x = self.random_generator.random_range(0..BOARD_WIDTH) as i32;
            let y = self.random_generator.random_range(0..BOARD_HEIGHT) as i32;
            let coord = Vector2i::from_tuple((x, y));
            if DrawBoardUtils::is_tile_empty(draw_tile_board, coord) {
                DrawBoardUtils::add_four_way_draw_tile(draw_tile_board, coord);
                blocks_placed += 1;

                let index = BoardCoordinate::from_vector2i(coord).to_index();
                let data_tile = &mut self.data[index];
                data_tile.make_traversable();
            }
        }
    }

    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
    // I tried creating a struct for managing all data tiles and encapsulating logic but hell broke loose
    // See commit @f2d5f5c4b7fb44413d1607c9f496ffff518ec7f5
    pub(crate) fn get_data_tile_mut(&mut self, coord: &BoardCoordinate) -> &mut DataTile {
        &mut self.data[coord.to_index()]
    }

    pub(crate) fn get_first_traversable_tile_coordinates_in_board(&self) -> Option<BoardCoordinate> {
        let index = self.data.iter().position(
            |tile| tile.is_traversable()
        );
        index.map(|idx| BoardCoordinate::from_index(idx))
    }

    pub(crate) fn get_random_traversable_tile_coordinates_in_board(&mut self) -> Option<BoardCoordinate> {
        let max_number_of_random_tries = 100;
        let mut number_of_random_tries = 0;
        let mut result: Option<BoardCoordinate> = Option::None;
        
        while result.is_none() && number_of_random_tries < max_number_of_random_tries {
            let random_idx = self.random_generator.random_range(0..self.data.len()-1);
            let tile = &self.data[random_idx];
            number_of_random_tries += 1;

            if tile.is_traversable() {
                result = Some(BoardCoordinate::from_index(random_idx))
            }
        }

        if result.is_none() {
            result = self.get_first_traversable_tile_coordinates_in_board();
        }
        
        result
    }

    pub(crate) fn get_graphics(&self) -> &Gd<DrawTileBoard> {
        &self.graphics
    }
}

