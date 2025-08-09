use godot::classes::*;
use godot::prelude::*;

use crate::board::data::data_tile::DataTile;
use crate::board::{constants::BOARD_SIZE, coordinate::{index_to_coordinate, BoardCoordinate}, entity::BoardEntity};

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct DataTileBoard {
    
    data: [DataTile<'static>; BOARD_SIZE],
    base: Base<Node>,
}

#[godot_api]
impl INode for DataTileBoard {
    fn init(base: Base<Node>) -> Self {
        Self {
            data: generate_empty_board_data(),
            base,
        }
    }
}

impl DataTileBoard {
    fn get_tile_ref(&self, coord: &BoardCoordinate) -> &DataTile {
        & self.data[coord.to_index()]
    }

    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
    fn get_tile_mut(&mut self, coord: &BoardCoordinate) -> &mut DataTile<'static> {
        &mut self.data[coord.to_index()]
    }
}

fn generate_empty_board_data<'a>() -> [DataTile<'a>; BOARD_SIZE as usize] {
    let data: [DataTile; BOARD_SIZE as usize] = core::array::from_fn(
        |i| DataTile::new(index_to_coordinate(i))
    );
    data
}