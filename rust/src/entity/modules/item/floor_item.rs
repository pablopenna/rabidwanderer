use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::get_item_factory_node_from_tree;
use crate::item::factory::ItemFactory;
use crate::item::item::Item;
use crate::item::item_definition::ItemDefinition;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FloorItem {
    item: ItemDefinition,
    factory: Option<Gd<ItemFactory>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for FloorItem {
    fn init(base: Base<Node>) -> Self {
        Self {
            item: ItemDefinition::Dummy,
            factory: Option::None,
            base
        }
    }

    fn ready(&mut self) {
        let node = self.base_mut().to_godot().upcast::<Node>();
        let factory = get_item_factory_node_from_tree(node);
        self.factory = Some(factory);
    }
}

impl FloorItem {
    fn set_item(&mut self, def: ItemDefinition) {
        self.item = def;
    }

    fn pickup(&mut self) -> Gd<Item>{
        let mut factory = self.factory.clone().unwrap();
        let factory = factory.bind_mut();
        factory.instance_item(&self.item)
    }
}
