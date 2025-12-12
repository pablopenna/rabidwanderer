use godot::prelude::*;
use rand::Rng;

#[derive(GodotConvert, Var, Export, Clone, PartialEq, Debug)]
#[godot(via = GString)]
pub(crate) enum SkillDefinition {
    Tackle,
    Bite,
    SonicPunch,
    LickWounds,
}

impl SkillDefinition {
    pub(crate) fn _get_display_name(&self) -> &'static str {
        match self {
            SkillDefinition::Tackle => "Tackle",
            SkillDefinition::Bite => "Bite",
            SkillDefinition::SonicPunch => "Sonic Punch",
            SkillDefinition::LickWounds => "Lick Wounds",
        }
    }

    pub(crate) fn get_description(&self) -> &'static str {
        match self {
            SkillDefinition::Tackle =>
                "[img]res://art/misc/tombstone.png[/img]Furiously charges at the enemy dealing average damage",
            SkillDefinition::Bite =>
                "Omn nom nom nom\n...\nChomp\n...\n*gulp*",
            SkillDefinition::SonicPunch =>
                "Fiaum!",
            SkillDefinition::LickWounds =>
                "Meow",
        }
    }

    pub(crate) fn get_cooldown(&self) -> u8 {
        match self {
            SkillDefinition::Tackle => 2,
            SkillDefinition::Bite => 1,
            SkillDefinition::SonicPunch => 1,
            SkillDefinition::LickWounds => 2,
        }
    }

    const _RANDOM_POOL: [SkillDefinition; 3] = [
        SkillDefinition::Tackle,
        SkillDefinition::Bite,
        SkillDefinition::SonicPunch,
    ];

    pub(crate) fn _random() -> Self {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..Self::_RANDOM_POOL.len());
        Self::_RANDOM_POOL[idx].clone()
    }

    pub(crate) fn from_gstring(item: GString) -> SkillDefinition {
        SkillDefinition::from_variant(&item.to_variant())
    }
}
