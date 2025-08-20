use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::get_item_factory_node_from_tree;
use crate::entity::board_entity::BoardEntity;
use crate::item::factory::ItemFactory;
use crate::item::item::Item;
use crate::item::item_definition::ItemDefinition;
use crate::utils::get_first_child_of_type::get_first_child_of_type;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FloorItemModule {
    entity: OnReady<Gd<BoardEntity>>,
    item: ItemDefinition,
    factory: OnReady<Gd<ItemFactory>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for FloorItemModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            entity: OnReady::manual(),
            item: ItemDefinition::Dummy,
            factory: OnReady::manual(),
            base
        }
    }

    fn ready(&mut self) {
        let node = self.base_mut().to_godot().upcast::<Node>();
        let factory = get_item_factory_node_from_tree(node);
        self.factory.init(factory);

        let parent = self.base_mut().to_godot().upcast::<Node>().get_parent().unwrap();
        self.entity.init(parent.cast::<BoardEntity>());
    }
}

impl FloorItemModule {
    pub(crate) fn set_item(&mut self, def: ItemDefinition) {
        self.item = def;
    }

    fn generate_picked_up_item(&mut self) -> Gd<Item>{
        let factory = self.factory.bind_mut();
        factory.instance_item(&self.item)
    }

    pub(crate) fn pickup(&mut self) -> Gd<Item> {
        let item = self.generate_picked_up_item();
        self.entity.call_deferred("queue_free", &[]);
        item
    }

    pub(crate) fn get_floor_item_from_entity(entity: Gd<BoardEntity>) -> Option<Gd<FloorItemModule>> {
        get_first_child_of_type::<FloorItemModule>(&entity)
    }
}
