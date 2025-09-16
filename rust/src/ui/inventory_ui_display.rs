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
        let mut display = TextureRect::new_alloc();
        
        let texture = item.bind().get_icon();
        if texture.is_some() {
            display.set_texture(&texture.unwrap())
        }

        display.signals().gui_input().connect_other(self, move |self_ref, event| {
            self_ref.on_display_gui_event(event, &item);
        });

        self.base_mut().add_child(&display);
    }

    fn on_display_gui_event(&mut self, event: Gd<InputEvent>, item: &Gd<Item>) {
        if event.is_pressed() {
            GlobalSignals::get_singleton().signals().inventory_ui_item_clicked().emit(item);
        }
    }
}
