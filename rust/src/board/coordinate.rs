use godot::builtin::Vector2i;

use crate::board::constants::BOARD_WIDTH;

pub(crate) struct BoardCoordinate {
    x: usize,
    y: usize,
}

pub(crate) fn godot_vector_to_vector2d(vector: Vector2i) -> BoardCoordinate {
    BoardCoordinate {
        x: vector.x as usize,
        y: vector.y as usize,
    }
}

pub(crate) fn index_to_coordinate(idx: usize) -> BoardCoordinate {
    BoardCoordinate {
        x: idx % BOARD_WIDTH,
        y: idx / BOARD_WIDTH,
    }
}

pub(crate) fn coordinate_to_index(coordinate: BoardCoordinate) -> usize {
    coordinate.y * BOARD_WIDTH + coordinate.x
}