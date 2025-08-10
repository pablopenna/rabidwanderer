use godot::builtin::Vector2i;

use crate::board::constants::{BOARD_HEIGHT, BOARD_WIDTH};

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

    pub(crate) fn from_index(idx: usize) -> BoardCoordinate {
        BoardCoordinate {
            x: idx % BOARD_WIDTH,
            y: idx / BOARD_WIDTH,
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        if self.x > BOARD_WIDTH-1 {
            return false;
        }
        if self.y > BOARD_HEIGHT-1 {
            return false;
        }
        true
    }
}
