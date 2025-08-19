use godot::classes::*;
use godot::prelude::*;

use crate::entity::board_entity::BoardEntity;
use crate::item::item::Item;

const MAX_NUMBER_OF_ITEMS: i32 = 8;

// Items are placed as children of this node
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct InventoryModule {
    base: Base<Node>,
}

impl InventoryModule {

    pub(crate) fn add_item(&mut self, item: Gd<Item>) {
        let item_node = item.upcast::<Node>();
        self.base_mut().add_child(&item_node);
    }

    pub(crate) fn has_room(&self) -> bool {
        self.base().get_child_count() < MAX_NUMBER_OF_ITEMS
    }

    pub(crate) fn _add_item_if_has_room(&mut self, item: Gd<Item>) -> bool {
        let mut result = false;
        if self.has_room() {
            self.add_item(item);
            result = true;
        }
        result
    }

    pub(crate) fn _remove_item(&mut self, index: usize) {
        let children = self.base().get_children();
        let child_to_remove = children.get(index);
        if child_to_remove.is_some() {
            let child_to_remove = child_to_remove.unwrap();
            self.base_mut().remove_child(&child_to_remove);
        }
    }

    fn _get_inventory_module_from(entity: Gd<BoardEntity>) -> Option<Gd<InventoryModule>> {
        let node = entity.upcast::<Node>();
        BoardEntity::get_first_child_of_type::<InventoryModule>(&node)
    }
}
