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
                .get_data_tile_mut(origin_coordinate)
                .clone();

        let target_data_tile = 
            &mut self
                .get_board()
                .unwrap()
                .bind_mut()
                .get_data_tile_mut(&target_coordinate)
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
            let mut board = self.get_board().unwrap();
            let mut binding = board.bind_mut();
            let data_tile = &mut binding.get_data_tile_mut(&coordinate).clone();
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
}

