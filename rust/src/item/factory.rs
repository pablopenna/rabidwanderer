use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::ITEM_FACTORY_GROUP;
use crate::item::item::Item;
use crate::item::item_definition::ItemDefinition;

/* 
* This is a template for an item.
* Each individual kind of Item will need to be defined in a dedicated scene.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct ItemFactory {
    #[export]
    dummy_item_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    rusty_gauntlet_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    rusty_plate_scene: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ItemFactory {
    fn init(base: Base<Node>) -> Self {
        Self {
            dummy_item_scene: OnEditor::default(),
            rusty_gauntlet_scene: OnEditor::default(),
            rusty_plate_scene: OnEditor::default(),
            base
        }
    }

    fn ready(&mut self) {
        let mut node = self.base_mut().to_godot().upcast::<Node>();
        node.add_to_group(ITEM_FACTORY_GROUP);
    }
}

impl ItemFactory {
    pub(crate) fn instance_item(&self, def: &ItemDefinition) -> Gd<Item> {
        let scene = self.get_scene_for_item(def);
        let item = scene.instantiate_as::<Item>();
        item
    }

    fn get_scene_for_item(&self, def: &ItemDefinition) -> Gd<PackedScene> {
        match def {
            ItemDefinition::Dummy => self.dummy_item_scene.get_property().unwrap(),
            ItemDefinition::RustyGauntlet => self.rusty_gauntlet_scene.get_property().unwrap(),
            ItemDefinition::RustyPlate => self.rusty_plate_scene.get_property().unwrap(),
        }
    }
}
