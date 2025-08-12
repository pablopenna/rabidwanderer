use godot::classes::*;
use godot::prelude::*;

use crate::board::coordinate::BoardCoordinate;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BoardEntity {
    coordinates: BoardCoordinate,
    base: Base<Node2D>,
}

impl BoardEntity {

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
