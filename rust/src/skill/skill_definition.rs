use godot::prelude::*;
use rand::Rng;

#[derive(GodotConvert, Var, Export, Clone)]
#[godot(via = GString)] 
pub(crate) enum SkillDefinition {
    Tackle,
    Bite,
}

impl SkillDefinition {
    pub(crate) fn get_display_name(&self) -> &'static str {
        match self {
            SkillDefinition::Tackle => "Tackle",
            SkillDefinition::Bite => "Bite",
        }
    }

    pub(crate) fn get_description(&self) -> &'static str {
        match self {
            SkillDefinition::Tackle => 
                "Furiously charges dealing average damage",
            SkillDefinition::Bite => 
                "Omn nom nom nom
                ...
                Chomp
                ...
                *gulp*
                ",
        }
    }

    const RANDOM_POOL: [SkillDefinition; 2] = [
        SkillDefinition::Tackle,
        SkillDefinition::Bite,
    ];

    pub(crate) fn random() -> Self {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..Self::RANDOM_POOL.len());
        Self::RANDOM_POOL[idx].clone()
    }

    pub(crate) fn from_gstring(item: GString) -> SkillDefinition {
        SkillDefinition::from_variant(&item.to_variant())
    }
}