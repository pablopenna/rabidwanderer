use godot::classes::*;
use godot::prelude::*;

use godot::obj::Gd;

use rand::Rng;

use crate::board::constants::BOARD_SIZE;
use crate::board::coordinate::BoardCoordinate;
use crate::board::data::data_tile::DataTile;

// Just a container node for DataTiles.
// I can leverage node.move_child to ensure a position of a tile as child is as expected
// https://docs.godotengine.org/en/stable/classes/class_node.html#class-node-method-move-child
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct DataTileBoard {
    base: Base<Node>
}

#[godot_api]
impl INode for DataTileBoard {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        self.generate_empty_board_data();
    }
}

impl DataTileBoard {
    fn get_all_data_tiles(&self) -> Array<Gd<DataTile>> {
        let children = self.base().get_children();
        let children: Array<Gd<DataTile>> = children.iter_shared().map(
            |n| n.cast::<DataTile>()
        ).collect();
        children
    }

    fn generate_empty_board_data(&mut self) {
        for idx in 0..BOARD_SIZE {
            let coord = BoardCoordinate::from_index(idx);
            let data_tile = DataTile::new(coord);
            self.base_mut().add_child(&data_tile);
        }
    }

    pub(crate) fn get_tile_at_index(&self, idx: usize) -> Option<Gd<DataTile>> {
        let child = self.base().get_child(idx as i32);
        if child.is_none() {
            return None;
        }

        let tile = child.unwrap().cast::<DataTile>();
        Some(tile)
    }

    pub(crate) fn get_tile_with_coordinate(&self, coord: BoardCoordinate) -> Option<Gd<DataTile>> {
        let children = self.base().get_children();
        let pos = children.iter_shared().position(|child| {
            let tile = child.cast::<DataTile>();
            let bind = tile.bind();
            let tile_coords = bind.get_coordinates();
            return *tile_coords == coord;
        });
        if pos.is_none() {
            return None;
        }
        let tile = children.at(pos.unwrap());
        let tile = tile.cast::<DataTile>();

        Some(tile)
    }

    pub(crate) fn get_first_traversable_tile_coordinates_in_board(&self) -> Option<BoardCoordinate> {
        let tiles = self.get_all_data_tiles();
        let index = tiles.iter_shared().position(
            |mut tile| tile.bind_mut().is_traversable()
        );
        index.map(|idx| BoardCoordinate::from_index(idx))
    }

    pub(crate) fn get_random_traversable_tile_coordinates_in_board(&mut self) -> Option<BoardCoordinate> {
        let tiles = self.get_all_data_tiles();
        let mut rng = rand::rng();
        let max_number_of_random_tries = 100;
        let mut number_of_random_tries = 0;
        let mut result: Option<BoardCoordinate> = Option::None;
        
        while result.is_none() && number_of_random_tries < max_number_of_random_tries {
            let random_idx = rng.random_range(0..tiles.len()-1);
            let tile = self.get_tile_at_index(random_idx);
            number_of_random_tries += 1;

            if tile.is_some() && tile.unwrap().bind().is_traversable() {
                result = Some(BoardCoordinate::from_index(random_idx))
            }
        }

        if result.is_none() {
            result = self.get_first_traversable_tile_coordinates_in_board();
        }
        
        result
    }
}
