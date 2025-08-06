use crate::board::board_coordinate::BoardCoordinate;

trait BoardEntity {
    fn get_coordinates(&self) -> BoardCoordinate;
    fn set_coordinates(&self, coord: BoardCoordinate);
}
