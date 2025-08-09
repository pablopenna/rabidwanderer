use godot::classes::*;
use godot::prelude::*;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::board::coordinate::coordinate_to_index;
use crate::board::coordinate::godot_vector_to_vector2d;
use crate::board::data::data_tile::generate_empty_board_data;
use crate::board::data::data_tile::DataTile;
use crate::board::graphics::draw_tile_board::DrawTileBoard;
use crate::board::graphics::utils as DrawBoardUtils;
use crate::board::constants::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub(crate) struct Board {
    random_generator: ThreadRng,
    data: [DataTile<'static>; BOARD_SIZE],
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
                
                let data_tile = &mut self.data[coordinate_to_index(godot_vector_to_vector2d(coord))];
                data_tile.make_traversable();
            }
        }
    }

    pub(crate) fn get_data_ref(&self) -> &[DataTile<'static>; BOARD_SIZE] {
        &self.data
    }

    pub(crate) fn get_data_mut(&mut self) -> &mut [DataTile<'static>; BOARD_SIZE] {
        &mut self.data
    }

    pub(crate) fn get_graphics(&self) -> &Gd<DrawTileBoard> {
        &self.graphics
    }
}

