use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::item::inventory::InventoryModule;
use crate::global_signals::GlobalSignals;

// Items are placed as children of this node
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct LinkInventoryToUiModule {
    base: Base<Node>,
    #[export]
    inventory: OnEditor<Gd<InventoryModule>>
}

#[godot_api]
impl INode for LinkInventoryToUiModule {
    fn ready(&mut self) {
        let inventory = self.inventory.get_property().unwrap();
        self.link(inventory);
    }
}

impl LinkInventoryToUiModule {
    fn link(&mut self, inventory: Gd<InventoryModule>) {
        inventory.signals().item_added().connect(|item| { 
            GlobalSignals::get_singleton().signals().inventory_ui_item_added().emit(&item);
        });
    }
}
