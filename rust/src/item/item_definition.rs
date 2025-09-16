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
    pub(crate) fn get_display_name(&self) -> &'static str {
        match self {
            ItemDefinition::Dummy => "Dummy",
            ItemDefinition::RustyGauntlet => "Rusty Gauntlet",
            ItemDefinition::RustyPlate => "Rusty Plate",
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

    pub(crate) fn from_gstring(item: GString) -> ItemDefinition {
        ItemDefinition::from_variant(&item.to_variant())
    }
}