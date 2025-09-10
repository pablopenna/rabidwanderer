use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub(crate) enum ModifierType {
    #[default]
    FLAT, // +10 attack
    PERCENTAGE, // +10% attack
    CUSTOM, // (base_attack) => *custom logic* => modified attack
}

impl ModifierType {
    pub(crate) fn from_gstring(mod_type: GString) -> ModifierType {
        ModifierType::from_variant(&mod_type.to_variant())
    }
}
