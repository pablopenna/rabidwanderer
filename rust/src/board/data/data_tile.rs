use std::{cell::RefCell, rc::Rc};

use godot::obj::Gd;

use crate::board::constants::BOARD_SIZE;
use crate::board::coordinate::BoardCoordinate;
use crate::entity::board_entity::BoardEntity;

#[derive(Clone)]
pub(crate) struct DataTile {
    _coordinates: BoardCoordinate,
    can_be_traversed: bool,
    // Read ADR-01 and ADR-02 for design decissions on why these attributes are done this way.
    entities: Vec<Rc<RefCell<Gd<BoardEntity>>>>,
}

impl DataTile {
    pub fn new(coordinates: BoardCoordinate) -> Self {
        Self {
            _coordinates: coordinates,
            can_be_traversed: false,
            entities: Vec::new(),
        }
    }

    pub(crate) fn _get_coordinates(&self) -> &BoardCoordinate {
        &self._coordinates
    }
    
    pub(crate) fn make_traversable(&mut self) {
        self.can_be_traversed = true;
    }

    pub(crate) fn is_traversable(&self) -> bool {
        self.can_be_traversed
    }

    pub(crate) fn add_entity(& mut self, entity: Rc<RefCell<Gd<BoardEntity>>>) {
        self.entities.push(entity);
    }

    pub(crate) fn remove_entity(& mut self, entity_to_remove: Rc<RefCell<Gd<BoardEntity>>>) -> bool {
        let position = self.entities.iter().position(
            |entity| Rc::ptr_eq(&entity, &entity_to_remove)
        );
        if position.is_none() {
            return false;
        }
        self.entities.remove(position.unwrap());
        return true;
    }
}

pub(crate) fn generate_empty_board_data() -> [DataTile; BOARD_SIZE as usize] {
    let data: [DataTile; BOARD_SIZE as usize] = core::array::from_fn(
        |i| DataTile::new(BoardCoordinate::from_index(i))
    );
    data
}

