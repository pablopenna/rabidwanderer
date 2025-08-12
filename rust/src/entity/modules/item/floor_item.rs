use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::get_item_factory_node_from_tree;
use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::item::inventory::InventoryModule;
use crate::item::factory::ItemFactory;
use crate::item::item::Item;
use crate::item::item_definition::ItemDefinition;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FloorItemModule {
    entity: Option<Gd<BoardEntity>>,
    item: ItemDefinition,
    factory: Option<Gd<ItemFactory>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for FloorItemModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            entity: Option::None,
            item: ItemDefinition::Dummy,
            factory: Option::None,
            base
        }
    }

    fn ready(&mut self) {
        let node = self.base_mut().to_godot().upcast::<Node>();
        let factory = get_item_factory_node_from_tree(node);
        self.factory = Some(factory);

        let parent = self.base_mut().to_godot().upcast::<Node>().get_parent().unwrap();
        self.entity = Some(parent.cast::<BoardEntity>());

        self.entity.clone().unwrap().signals().on_interact().connect_other(self, Self::on_interact_with);
    }
}

impl FloorItemModule {
    pub(crate) fn set_item(&mut self, def: ItemDefinition) {
        self.item = def;
    }

    pub(crate) fn generate_picked_up_item(&self) -> Gd<Item>{
        let mut factory = self.factory.clone().unwrap();
        let factory = factory.bind_mut();
        factory.instance_item(&self.item)
    }

    fn get_inventory_module_from(entity: Gd<BoardEntity>) -> Option<Gd<InventoryModule>> {
        let node = entity.upcast::<Node>();
        BoardEntity::get_first_child_of_type::<InventoryModule>(&node)
    }

    fn on_interact_with(&mut self, mut this_entity: Gd<BoardEntity>, picker_entity: Gd<BoardEntity>) {
        godot_print!("interacting...");
        let inventory_from_picker_entity = FloorItemModule::get_inventory_module_from(picker_entity);
        if inventory_from_picker_entity.is_none() {
            return;
        }

        let item = self.generate_picked_up_item();
        let has_been_picked_up = inventory_from_picker_entity.unwrap().bind_mut().add_item_if_has_room(item);

        if has_been_picked_up {
            godot_print!("Picked up!");
            this_entity.queue_free();
        }
    }
}
