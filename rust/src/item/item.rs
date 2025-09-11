use godot::classes::*;
use godot::prelude::*;

use crate::item::item_definition::ItemDefinition;
use crate::stats::modifier::modifier::StatModifier;
use crate::stats::stat::Stat;

/* 
* This is a template for an item.
* Each individual kind of Item will need to be defined in a dedicated scene.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Item {
    #[export]
    name: ItemDefinition,
    #[export]
    icon: OnEditor<Gd<Texture2D>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Item {
    fn init(base: Base<Node>) -> Self {
        Self {
            name: ItemDefinition::Dummy, // default, to be overriden later
            icon: OnEditor::default(),
            base
        }
    }
}

#[godot_api]
impl Item {
    pub(crate) fn get_all_modifiers(&self) -> Array<Gd<StatModifier>> {
        self.base().get_children().iter_shared().map(|child| child.cast::<StatModifier>()).collect()
    }

    pub(crate) fn get_modifiers_for_stat(&self, stat: Stat) -> Array<Gd<StatModifier>> {
        let mods = self.get_all_modifiers();
        mods.iter_shared().filter(|r#mod| Stat::from_gstring(r#mod.bind().get_stat()) == stat).collect()
    }
}
