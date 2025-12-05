use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, PartialEq, Debug)]
#[godot(via = GString)]
pub(crate) enum TargetFaction {
    OneSelf,
    Opponent,
    AllyExcludingSelf,
    AllyIncludingSelf,
}

impl TargetFaction {
    pub(crate) fn from_gstring(r#type: GString) -> TargetFaction {
        TargetFaction::from_variant(&r#type.to_variant())
    }
}
