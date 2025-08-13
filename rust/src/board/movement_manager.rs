use std::cell::RefCell;
use std::rc::Rc;

use godot::prelude::*;

use crate::consts::groups::MOVEMENT_MANAGER_GROUP;
use crate::board::board::Board;
use crate::board::coordinate::BoardCoordinate;
use crate::entity::board_entity::BoardEntity;

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
        // Adding to group crashes if called from within init()
        // self.base_mut().add_to_group_ex(MOVEMENT_MANAGER_GROUP).persistent(true).done();
        self.base_mut().add_to_group(MOVEMENT_MANAGER_GROUP);
    }
}

#[godot_api]
impl BoardMovementManager {

    pub(crate) fn move_entity_in_board(&mut self, entity_reference: Rc<RefCell<Gd<BoardEntity>>>, board_movement: Vector2i) {
        let mut entity_borrow = entity_reference.borrow_mut();
        let mut entity = entity_borrow.bind_mut();

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
        
        origin_data_tile.remove_entity(entity_reference.clone());
        target_data_tile.add_entity(entity_reference.clone());
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

    pub(crate) fn add_entity_to_board_at_coordinate(&mut self, entity_reference: Rc<RefCell<Gd<BoardEntity>>>, coordinate: BoardCoordinate) {
        {
            let mut board = self.get_board().unwrap();
            let mut board_binding = board.bind_mut();
            let data_tile = board_binding.get_data_tile_mut(&coordinate);
            data_tile.add_entity(entity_reference.clone());
        }

        {
            let mut entity_borrow = entity_reference.borrow_mut();
            let mut entity = entity_borrow.bind_mut();
            entity.set_coordinates(coordinate.clone());
        
            let board = self.get_board().unwrap();
            let board_binding = board.bind();
            let draw_tile_board = board_binding.get_graphics();
            let position = draw_tile_board.map_to_local(coordinate.to_godot_vector2i());
            
            entity.set_world_position(position);
        }
    }

    fn process_interaction_of_entity_with_tile(
        &mut self,
        entity_reference: Rc<RefCell<Gd<BoardEntity>>>,
        coordinate: BoardCoordinate,
    ) {
        
        let mut board = self.get_board().unwrap();
        let mut board = board.bind_mut();
        let data_tile = board.get_data_tile_mut(&coordinate);
        
        let entities_to_interact_with: Vec<_> = data_tile.get_entities();
        let entities_to_interact_with: Vec<_> = entities_to_interact_with
            .iter()
            .filter(|e| !Rc::ptr_eq(e, &entity_reference))
            .collect();
        
        if entities_to_interact_with.is_empty() {
            return;
        }
        
        let mut entities_to_interact_with: Vec<_> = entities_to_interact_with
            .iter()
            .map(|e| e.borrow_mut())
            .collect();
        
        let main_entity = entity_reference.borrow_mut();
        
        entities_to_interact_with
            .iter_mut()
            .for_each(|e| e.bind_mut().interact_with(&main_entity));
    }
}


