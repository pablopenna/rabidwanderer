use godot::prelude::*;
use rand::Rng;

#[derive(GodotConvert, Var, Export, Clone)]
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

    const RANDOM_POOL: [ItemDefinition; 2] = [
        ItemDefinition::RustyGauntlet,
        ItemDefinition::RustyPlate,
    ];

    pub(crate) fn random() -> Self {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..Self::RANDOM_POOL.len());
        Self::RANDOM_POOL[idx].clone()
    }
}