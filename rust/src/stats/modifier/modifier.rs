use godot::classes::*;
use godot::prelude::*;

// To be placed as children of Items.
// Items will get their stats modifications 
// from these.
#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct StatModifier {
    base: Base<Node>
}

#[godot_api]
impl StatModifier {
    
}