use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub(crate) enum ModifierType {
    #[default]
    FLAT, // +10 attack
    PERCENTAGE, // +10% attack
    CUSTOM, // (base_attack) => *custom logic* => modified attack
}