use godot::builtin::Vector2i;

use crate::board::constants::BOARD_WIDTH;

#[derive(Default)]
#[derive(Clone)]
pub(crate) struct BoardCoordinate {
    x: usize,
    y: usize,
}

impl BoardCoordinate {

    pub(crate) fn to_godot_vector2i(&self) -> Vector2i {
        Vector2i {x: self.x as i32, y: self.y as i32}
    }

    pub(crate) fn to_index(&self) -> usize {
        self.y * BOARD_WIDTH + self.x
    }

    pub(crate) fn from_vector2i(vector: Vector2i) -> Self {
        Self { x: vector.x as usize, y: vector.y as usize }
    }
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
