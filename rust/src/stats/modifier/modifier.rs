use godot::classes::*;
use godot::prelude::*;

use crate::stats::modifier::custom::definition::CustomModifier;
use crate::stats::modifier::r#type::ModifierType;
use crate::stats::stat::Stat;

// To be placed as children of Items.
// Items will get their stats modifications 
// from these.
#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct StatModifier {
    base: Base<Node>,
    #[export]
    stat: Stat,
    #[export]
    mod_type: ModifierType,
    #[export]
    custom_implementation: CustomModifier,
    #[export]
    value: f32,
}

#[godot_api]
impl StatModifier {
    
}