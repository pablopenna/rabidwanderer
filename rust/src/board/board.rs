use godot::classes::*;
use godot::prelude::*;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::board::coordinate::BoardCoordinate;
use crate::board::data::data_tile::DataTile;
use crate::board::data::data_tile_board::DataTileBoard;
use crate::board::graphics::draw_tile_board::DrawTileBoard;
use crate::board::graphics::utils as DrawBoardUtils;
use crate::board::constants::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub(crate) struct Board {
    random_generator: ThreadRng,
    data: Gd<DataTileBoard>,
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
            data: DataTileBoard::new_alloc(),
            graphics: DrawTileBoard::new_alloc(),
            tile_set: OnEditor::default(),
            base,
        }
    }

    fn ready(&mut self) {
        let tile_set = self.tile_set.to_godot();
        self.get_graphics().set_tile_set(&tile_set);
        
        let graphics_node = self.get_graphics().upcast::<Node>();
        self.base_mut().add_child(&graphics_node);

        let data_node = self.get_data().upcast::<Node>();
        self.base_mut().add_child(&data_node);

        self.populate_board();
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
        let blocks_to_place: u8 = BOARD_SIZE as u8 - 5;
        let mut blocks_placed: u8 = 0;
        let draw_tile_board = &mut self.graphics.clone().upcast::<TileMapLayer>();
        let data_tile_board = &mut self.data;

        while blocks_placed < blocks_to_place {
            let x = self.random_generator.random_range(0..BOARD_WIDTH) as i32;
            let y = self.random_generator.random_range(0..BOARD_HEIGHT) as i32;
            let coord = Vector2i::from_tuple((x, y));
            if DrawBoardUtils::is_tile_empty(draw_tile_board, coord) {
                DrawBoardUtils::add_four_way_draw_tile(draw_tile_board, coord);
                blocks_placed += 1;

                let index = BoardCoordinate::from_vector2i(coord).to_index();
                let mut data_tile = data_tile_board.bind().get_tile_at_index(index).unwrap();
                data_tile.bind_mut().make_traversable();
            }
        }
    }

    pub(crate) fn get_tile_at(&mut self, coord: &BoardCoordinate) -> Option<Gd<DataTile>> {
        self.data.bind_mut().get_tile_at_index(coord.to_index())
    }

    pub(crate) fn get_first_traversable_tile_in_board(&mut self) -> Option<Gd<DataTile>> {
        let coord = self.data.bind_mut().get_first_traversable_tile_coordinates_in_board();
        if coord.is_none() {
            return None;
        }
        self.get_tile_at(&coord.unwrap())
    }

    pub(crate) fn get_random_traversable_tile_in_board(&mut self) -> Option<Gd<DataTile>> {
        let coord = self.data.bind_mut().get_random_traversable_tile_coordinates_in_board();
        if coord.is_none() {
            return None;
        }
        self.get_tile_at(&coord.unwrap())
    }

    pub(crate) fn get_graphics(&self) -> Gd<DrawTileBoard> {
        self.graphics.clone()
    }

    fn get_data(&self) -> Gd<DataTileBoard> {
        self.data.clone()
    }
}

