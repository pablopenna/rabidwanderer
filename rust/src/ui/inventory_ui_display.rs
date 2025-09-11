use godot::classes::*;
use godot::prelude::*;

use crate::global_signals::GlobalSignals;
use crate::item::item::Item;

// Items are placed as children of this node
#[derive(GodotClass)]
#[class(init, base=GridContainer)]
pub struct InventoryUiDisplay {
    base: Base<GridContainer>,
}

#[godot_api]
impl IGridContainer for InventoryUiDisplay {
    fn ready(&mut self) {
        self.setup();
    }
}

impl InventoryUiDisplay {
    fn setup(&mut self) {
        GlobalSignals::get_singleton().signals().inventory_ui_item_added().connect_other(self, Self::on_item_added_to_inventory);
    }

    fn on_item_added_to_inventory(&mut self, item: Gd<Item>) {
        let texture = item.bind().get_icon();
        let mut display = TextureRect::new_alloc();

        if texture.is_some() {
            display.set_texture(&texture.unwrap())
        }

        self.base_mut().add_child(&display);
    }
}
