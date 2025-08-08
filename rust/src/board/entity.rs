use godot::builtin::Vector2;

use crate::board::coordinate::BoardCoordinate;

// https://doc.rust-lang.org/book/ch10-02-traits.html
pub(crate) trait BoardEntity {
    fn get_coordinates(&self) -> &BoardCoordinate;
    fn set_coordinates(&mut self, coord: BoardCoordinate);
    fn set_world_position(&mut self, position: Vector2);
}
