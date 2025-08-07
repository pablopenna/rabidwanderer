use godot::builtin::Vector2i;

use crate::board::constants::BOARD_WIDTH;

pub(crate) struct BoardCoordinate {
    x: usize,
    y: usize,
}

impl BoardCoordinate {
    pub(crate) fn get_x(&self) -> usize {
        self.x
    }
    
    pub(crate) fn get_y(&self) -> usize {
        self.y
    }

    pub(crate) fn to_godot_vector2i(&self) -> Vector2i {
        Vector2i {x: self.x as i32, y: self.y as i32}
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