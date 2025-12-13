use godot::prelude::*;
use rand::Rng;

use crate::targeting::{target_amount::TargetAmount, target_faction::TargetFaction};

#[derive(GodotConvert, Var, Export, Clone, PartialEq, Debug, Default)]
#[godot(via = GString)]
pub(crate) enum SkillDefinition {
    #[default]
    Idle,
    Tackle,
    Bite,
    SonicPunch,
    LickWounds,
    SummonSpirit,
}

impl SkillDefinition {
    pub(crate) fn _get_display_name(&self) -> &'static str {
        match self {
            SkillDefinition::Idle => "Idle",
            SkillDefinition::Tackle => "Tackle",
            SkillDefinition::Bite => "Bite",
            SkillDefinition::SonicPunch => "Sonic Punch",
            SkillDefinition::LickWounds => "Lick Wounds",
            SkillDefinition::SummonSpirit => "Summon Spirit",
        }
    }

    pub(crate) fn get_description(&self) -> &'static str {
        match self {
            SkillDefinition::Idle => "Catch your breath",
            SkillDefinition::Tackle =>
                "[img]res://art/misc/tombstone.png[/img]Furiously charges at the enemy dealing average damage",
            SkillDefinition::Bite =>
                "Omn nom nom nom\n...\nChomp\n...\n*gulp*",
            SkillDefinition::SonicPunch =>
                "Fiaum!",
            SkillDefinition::LickWounds =>
                "Meow",
            SkillDefinition::SummonSpirit => "Share the burden",
        }
    }

    pub(crate) fn get_cooldown(&self) -> u8 {
        match self {
            SkillDefinition::Idle => 0,
            SkillDefinition::Tackle => 2,
            SkillDefinition::Bite => 2,
            SkillDefinition::SonicPunch => 1,
            SkillDefinition::LickWounds => 2,
            SkillDefinition::SummonSpirit => 3,
        }
    }

    pub(crate) fn get_target_amount(&self) -> TargetAmount {
        match self {
            SkillDefinition::Idle => TargetAmount::Single,
            SkillDefinition::Tackle => TargetAmount::All,
            SkillDefinition::Bite => TargetAmount::Single,
            SkillDefinition::SonicPunch => TargetAmount::Single,
            SkillDefinition::LickWounds => TargetAmount::Single,
            SkillDefinition::SummonSpirit => TargetAmount::Single,
        }
    }

    pub(crate) fn get_target_faction(&self) -> TargetFaction {
        match self {
            SkillDefinition::Idle => TargetFaction::OneSelf,
            SkillDefinition::Tackle => TargetFaction::Opponent,
            SkillDefinition::Bite => TargetFaction::Opponent,
            SkillDefinition::SonicPunch => TargetFaction::Opponent,
            SkillDefinition::LickWounds => TargetFaction::OneSelf,
            SkillDefinition::SummonSpirit => TargetFaction::OneSelf,
        }
    }

    pub(crate) fn get_priority(&self) -> i32 {
        match self {
            SkillDefinition::Idle => 0,
            SkillDefinition::Tackle => 0,
            SkillDefinition::Bite => 0,
            SkillDefinition::SonicPunch => 1,
            SkillDefinition::LickWounds => 0,
            SkillDefinition::SummonSpirit => 0,
        }
    }

    const _RANDOM_POOL: [SkillDefinition; 5] = [
        SkillDefinition::Tackle,
        SkillDefinition::Bite,
        SkillDefinition::SonicPunch,
        SkillDefinition::LickWounds,
        SkillDefinition::SummonSpirit,
    ];

    pub(crate) fn _random() -> Self {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..Self::_RANDOM_POOL.len());
        Self::_RANDOM_POOL[idx].clone()
    }

    pub(crate) fn _from_gstring(item: GString) -> SkillDefinition {
        SkillDefinition::from_variant(&item.to_variant())
    }
}
