use godot::prelude::*;

use crate::board::coordinate::BoardCoordinate;
use crate::board::entity::BoardEntity;
use crate::board::board::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct BoardMovementManager {
    base: Base<Node>,
    #[export]
    board: OnEditor<Gd<Board>>,
}

#[godot_api]
impl INode for BoardMovementManager {
    fn ready(&mut self) {
        //self.signals().entity_move_intent().connect_self(Self::on_entity_move_intent);
        // https://godot-rust.github.io/book/register/signals.html#connecting-from-outside
        self.board.signals().board_setted_up().connect_other(self, Self::on_board_setted_up);
    }
}

#[godot_api]
impl BoardMovementManager {

    pub(crate) fn move_entity_in_board(&mut self, &_entity: &mut impl BoardEntity, _board_movement: Vector2i) {
        
    }

    pub(crate) fn add_entity_to_board_at_coordinate(&mut self, &entity: &mut impl BoardEntity, coordinate: BoardCoordinate) {
        {
            let mut board = self.get_board().unwrap();
            let mut binding = board.bind_mut();
            let mut data = binding.get_data_mut().clone();
            let data_tile = &mut data[coordinate.to_index()];
            data_tile.add_entity(entity);
        }

        {
            entity.set_coordinates(coordinate.clone());
        }

        {
            let board = self.get_board().unwrap();
            let board_binding = board.bind();
            let draw_tile_board = board_binding.get_graphics();
            let position = draw_tile_board.map_to_local(coordinate.to_godot_vector2i());
            entity.set_world_position(position);
        }
    }

    fn on_board_setted_up(&mut self, &mut _board: Gd<Board>) {
        let coord = self.get_first_traversable_tile_coordinates_in_board();
        godot_print!("First available cell! {}", coord.unwrap().to_string());
    }

    // TODO: move to a more adequate file
    pub(crate) fn get_first_traversable_tile_coordinates_in_board(&mut self) -> Option<Vector2i> {
        let mut board_gd = self.get_board().to_godot().unwrap();
        let board = board_gd.bind_mut();
        let board_data = board.get_data();
        let index = board_data.iter().position(
            |tile| tile.is_traversable()
        );
        if index.is_none() {
            return None;
        }
        let tile = &board_data[index.unwrap()];
        let coordinates = tile.get_coordinates();
        return Option::Some(coordinates.to_godot_vector2i());
    }
}

