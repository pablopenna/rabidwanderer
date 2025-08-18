use godot::prelude::*;

use crate::board::data::data_tile::DataTile;
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

    pub(crate) fn move_entity_in_board(&mut self, entity: &mut Gd<BoardEntity>, board_movement: Vector2i) {
        let target_coordinate = BoardCoordinate::from_vector2i(
            entity.bind_mut().get_coordinates().to_godot_vector2i() + board_movement
        );

        if !target_coordinate.is_valid() {
            return;
        }

        let mut target_data_tile = 
            self
                .get_board()
                .unwrap()
                .bind_mut()
                .get_tile_at(&target_coordinate)
                .unwrap();
        
        if !target_data_tile.bind().is_traversable() {
            return;
        }
        
        DataTile::move_entity_to(entity, &mut target_data_tile);
        entity.bind_mut().set_coordinates(target_coordinate.clone());

        let target_world_position = 
            self
            .get_board()
            .unwrap()
            .bind()
            .get_graphics()
            .map_to_local(target_coordinate.to_godot_vector2i());
        entity.bind_mut().set_world_position(target_world_position);

        self.process_interaction_of_entity_with_tile(entity, target_coordinate);
    }

    pub(crate) fn add_entity_to_board_at_coordinate(&mut self, entity: &mut Gd<BoardEntity>, coordinate: BoardCoordinate) {
        let mut data_tile = self
            .get_board()
            .unwrap()
            .bind_mut()
            .get_tile_at(&coordinate)
            .unwrap();
        if entity.get_parent().is_none() {
            data_tile.bind_mut().add_entity_to_tile(entity);
        }
        DataTile::move_entity_to_deferred(entity, &mut data_tile);
        
        entity.bind_mut().set_coordinates(coordinate.clone());

        let world_position = self
            .get_board()
            .unwrap()
            .bind_mut()
            .get_graphics()
            .map_to_local(coordinate.clone().to_godot_vector2i());
        
        entity.bind_mut().set_world_position(world_position);
    }

    fn process_interaction_of_entity_with_tile(
        &mut self,
        entity: &mut Gd<BoardEntity>,
        coordinate: BoardCoordinate,
    ) {
        let mut data_tile = self.get_board().unwrap().bind_mut().get_tile_at(&coordinate).unwrap();
        
        let entities_to_interact_with = data_tile.bind_mut().get_entities();
        let entities_to_interact_with: Array<_> = entities_to_interact_with
            .iter_shared()
            .filter(|e| *e != *entity )
            .collect();
        
        if entities_to_interact_with.is_empty() {
            return;
        }
        
        entities_to_interact_with
            .iter_shared()
            .for_each(|mut e| e.bind_mut().interact_with(&entity));
    }
}


