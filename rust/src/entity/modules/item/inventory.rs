use godot::classes::*;
use godot::prelude::*;

use crate::item::item::Item;

const MAX_NUMBER_OF_ITEMS: usize = 8;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct InventoryModule {
    _items: [Option<Item>; MAX_NUMBER_OF_ITEMS],
    base: Base<Node>,
}

#[godot_api]
impl INode for InventoryModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            _items: std::array::from_fn(|_| None),
            base
        }
    }
}

impl InventoryModule {
    fn get_available_index(&self) -> Option<usize> {
        self._items.iter().position(|item| item.is_none())
    }

    fn add_item(&mut self, item: Item) {
        let pos = self.get_available_index().unwrap();
        self._items[pos] = Some(item);
    }

    pub(crate) fn has_room(&self) -> bool {
        self.get_available_index().is_some()
    }

    pub(crate) fn add_item_if_has_room(&mut self, item: Item) -> bool {
        let mut result = false;
        if self.has_room() {
            self.add_item(item);
            result = true;
        }
        result
    }

    pub(crate) fn remove_item(&mut self, index: usize) {
        self._items[index] = Option::None;
    }
}
