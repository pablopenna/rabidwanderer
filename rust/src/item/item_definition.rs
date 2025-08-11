use godot::prelude::*;

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)] 
pub(crate) enum ItemDefinition {
    Dummy,
    RustyGauntlet,
    RustyPlate,
}

impl ItemDefinition {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemDefinition::Dummy => "dummy",
            ItemDefinition::RustyGauntlet => "rusty gauntlet",
            ItemDefinition::RustyPlate => "rusty plate",
        }
    }
}