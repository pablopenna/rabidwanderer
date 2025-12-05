use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, PartialEq, Debug)]
#[godot(via = GString)] 
pub(crate) enum TargetAmount {
    Single,
    All
}

impl TargetAmount {
    pub(crate) fn from_gstring(r#type: GString) -> TargetAmount {
        TargetAmount::from_variant(&r#type.to_variant())
    }
}
