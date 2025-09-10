use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub(crate) enum CustomModifier {
    #[default]
    ONE, // outputs 1
}

impl CustomModifier {
    pub(crate) fn from_gstring(custom_mod: GString) -> CustomModifier {
        CustomModifier::from_variant(&custom_mod.to_variant())
    }
}
