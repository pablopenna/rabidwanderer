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

    pub(crate) fn move_entity_in_board(&mut self, &entity: &mut impl BoardEntity, board_movement: Vector2i) {
        let origin_coordinate = entity.get_coordinates();
        let target_coordinate = BoardCoordinate::from_vector2i(
            entity.get_coordinates().to_godot_vector2i() + board_movement
        );
        if !target_coordinate.is_valid() {
            return;
        }

        let origin_data_tile = 
            &mut self
                .get_board()
                .unwrap()
                .bind_mut()
                .get_data_mut()
                .bind_mut()
                .get_tile_mut(origin_coordinate)
                .clone();

        let target_data_tile = 
            &mut self
                .get_board()
                .unwrap()
                .bind_mut()
                .get_data_mut()
                .bind_mut()
                .get_tile_mut(origin_coordinate)
                .clone();
        
        if !target_data_tile.is_traversable() {
            return;
        }
        
        origin_data_tile.remove_entity(entity);
        target_data_tile.add_entity(entity);
        entity.set_coordinates(target_coordinate.clone());

        let target_world_position = 
            self
            .get_board()
            .unwrap()
            .bind()
            .get_graphics()
            .map_to_local(target_coordinate.to_godot_vector2i());
        entity.set_world_position(target_world_position);
    }

    pub(crate) fn add_entity_to_board_at_coordinate(&mut self, &entity: &mut impl BoardEntity, coordinate: BoardCoordinate) {
        {
            let data_tile = &mut self
                .get_board()
                .unwrap()
                .bind_mut()
                .get_data_mut()
                .bind_mut()
                .get_tile_mut(&coordinate)
                .clone();
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
        let binding = self
        .get_board()
        .unwrap();
        let binding = binding
        .bind();
        let binding = binding
        .get_data_ref()
        .bind();
        let tile = binding
        .get_first_traversable_tile();

        tile.map(|t| t.get_coordinates().to_godot_vector2i())
    }
}

