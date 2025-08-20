use godot::classes::*;
use godot::prelude::*;

use crate::board::coordinate::BoardCoordinate;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BoardEntity {
    coordinates: BoardCoordinate,
    base: Base<Node2D>,
}

#[godot_api]
impl BoardEntity {

    #[signal]
    pub(crate) fn on_interact(self_reference: Gd<BoardEntity>, interacted_with: Gd<BoardEntity>);

    #[signal]
    pub(crate) fn moved_board_tile();

    #[signal]
    pub(crate) fn added_to_board();

    pub(crate) fn get_coordinates(&self) -> &BoardCoordinate {
        &self.coordinates
    }
    
    pub(crate) fn set_coordinates(&mut self, coord: BoardCoordinate) {
        self.coordinates = coord;
    }

    pub(crate) fn set_world_position(&mut self, position: Vector2) {
        // let global_position = self.base().to_godot().to_global(position);
        self.base_mut().to_godot().set_position(position);
    }
}


