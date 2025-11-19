use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, PartialEq, Debug)]
#[godot(via = GString)] 
pub(crate) enum TargetingMode {
    FirstAvailable,
}

pub(crate) fn from_gstring(item: GString) -> TargetingMode {
    TargetingMode::from_variant(&item.to_variant())
}