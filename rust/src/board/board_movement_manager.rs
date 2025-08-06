use godot::prelude::*;

use crate::board::board::Board;
use crate::player::player::Player;

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
        self.signals().entity_move_intent().connect_self(Self::on_entity_move_intent);
    }
}

#[godot_api]
impl BoardMovementManager {
    #[signal]
    pub fn entity_move_intent(&mut entity_moving: Gd<Player>, board_movement: Vector2);

    fn on_entity_move_intent(&mut self, &mut entity_moving: Gd<Player>, board_movement: Vector2) {
        godot_print!("move intent signal!");
        let position = entity_moving.get_position();
        entity_moving.set_position(position + board_movement);
    }
}

