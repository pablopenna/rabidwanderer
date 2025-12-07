use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;

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

    pub(crate) fn get_entities_belonging_to_faction(
        reference_actor_team: &Team,
        faction: &TargetFaction,
        candidates: &Array<Gd<BattleEntity>>,
    ) -> Array<Gd<BattleEntity>> {
        let team = Self::get_team_for_faction(reference_actor_team, faction);

        Team::get_entities_from_team(&team, candidates)
    }

    // factions are relative, teams are absolute
    fn get_team_for_faction(reference_actor_team: &Team, faction: &TargetFaction) -> Team {
        match reference_actor_team {
            Team::Player => match faction {
                TargetFaction::Opponent => Team::Enemy,
                TargetFaction::OneSelf => Team::Player,
                TargetFaction::AllyExcludingSelf => Team::Player,
                TargetFaction::AllyIncludingSelf => Team::Player,
            },
            Team::Enemy => match faction {
                TargetFaction::Opponent => Team::Player,
                TargetFaction::OneSelf => Team::Enemy,
                TargetFaction::AllyExcludingSelf => Team::Enemy,
                TargetFaction::AllyIncludingSelf => Team::Enemy,
            },
        }
    }
}
