use crate::{board::{constants::BOARD_SIZE, coordinate::{index_to_coordinate, BoardCoordinate}, entity::BoardEntity}, player::player::Player};

pub(crate) struct DataTile<'a> {
    coordinates: BoardCoordinate,
    can_be_traversed: bool,
    // https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
    entities: Vec<&'a dyn BoardEntity>,
}

// impl<'a> = declares a lifetime parameter 'a for this impl block.
// DataTile<'a> = this impl is for DataTile instances that are parameterized with 'a.
impl<'a> DataTile<'a> {
    pub fn new(coordinates: BoardCoordinate) -> Self {
        Self {
            coordinates,
            can_be_traversed: false,
            entities: Vec::new(),
        }
    }
    
    pub(crate) fn make_traversable(&mut self) {
        self.can_be_traversed = true;
    }

    pub(crate) fn make_non_traversable(&mut self) {
        self.can_be_traversed = false;
    }

    pub(crate) fn is_traversable(&mut self) -> bool {
        self.can_be_traversed
    }

    pub(crate) fn add_entity(& mut self, entity: &'a impl BoardEntity) {
        self.entities.push(entity);
    }

    pub(crate) fn remove_entity(& mut self, entity_to_remove: &'a impl BoardEntity) -> bool {
        let position = self.entities.iter().position(
            |&entity| std::ptr::addr_eq(entity, entity_to_remove)
        );
        if position.is_none() {
            return false;
        }
        self.entities.remove(position.unwrap());
        return true;
    }
}

pub(crate) fn generate_empty_board_data<'a>() -> [DataTile<'a>; BOARD_SIZE as usize] {
    let data: [DataTile; BOARD_SIZE as usize] = core::array::from_fn(
        |i| DataTile::new(index_to_coordinate(i))
    );
    data
}

