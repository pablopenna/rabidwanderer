use godot::classes::*;
use godot::prelude::*;

use crate::item::item_definition::ItemDefinition;

/* 
* This is a template for an item.
* Each individual kind of Item will need to be defined in a dedicated scene.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Item {
    #[export]
    definition: ItemDefinition,
    base: Base<Node>,
}

#[godot_api]
impl INode for Item {
    fn init(base: Base<Node>) -> Self {
        Self {
            definition: ItemDefinition::Dummy, // default, to be overriden later
            base
        }
    }
}
