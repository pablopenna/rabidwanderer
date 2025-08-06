use crate::board::{board_coordinate::{index_to_coordinate, BoardCoordinate}, constants::BOARD_SIZE};

pub(crate) struct DataTile {
    pub(crate) coordinates: BoardCoordinate,
    pub(crate) can_be_traversed: bool,
}

impl DataTile {
    pub(crate) fn make_traversable(&mut self) {
        self.can_be_traversed = true;
    }
}

pub(crate) fn generate_empty_board_data() -> [DataTile; BOARD_SIZE as usize] {
    let data: [DataTile; BOARD_SIZE as usize] = core::array::from_fn(|i| DataTile {
        coordinates: index_to_coordinate(i),
        can_be_traversed: false,
    });
    data
}

