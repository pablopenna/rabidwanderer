use godot::prelude::*;

use crate::board::board::*;
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
        // https://godot-rust.github.io/book/register/signals.html#connecting-from-outside
        self.board.signals().board_setted_up().connect_other(self, Self::on_board_setted_up);
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

    fn on_board_setted_up(&mut self, &mut _board: Gd<Board>) {
        let coord = self.get_first_traversable_tile_coordinates_in_board();
        godot_print!("First available cell! {}", coord.unwrap().to_string());
    }

    fn get_first_traversable_tile_coordinates_in_board(&mut self) -> Option<Vector2i> {
        let mut board_gd = self.get_board().to_godot().unwrap();
        let mut board = board_gd.bind_mut();
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

