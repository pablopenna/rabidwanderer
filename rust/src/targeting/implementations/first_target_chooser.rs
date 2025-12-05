use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_chooser::TargetChooser;
use crate::targeting::target_faction::TargetFaction;

pub(crate) struct FirstTargetChooser {}

impl TargetChooser for FirstTargetChooser {
    fn choose_targets(
        actor: &Gd<BattleEntity>,
        candidates: &Array<Gd<BattleEntity>>,
        target_amount: &TargetAmount,
        target_faction: &TargetFaction,
    ) -> Array<Gd<BattleEntity>> {
        let actor_team = Team::from_gstring(actor.bind().get_team());
        let entities_in_target_faction =
            get_entities_belonging_to_faction(&actor_team, target_faction, candidates);
        
        if *target_amount == TargetAmount::All {
            return entities_in_target_faction;
        }
        
        let first_available_target = entities_in_target_faction
            .iter_shared()
            .find(|c| is_entity_alive(c) && are_entities_different(c, actor));
        
        let mut targets: Array<Gd<BattleEntity>> = array!();
        if first_available_target.is_some() {
            targets.push(&first_available_target.unwrap());
        }

        targets
    }
}

fn are_entities_different(actor_a: &Gd<BattleEntity>, actor_b: &Gd<BattleEntity>) -> bool {
    *actor_a != *actor_b
}

fn is_entity_alive(actor: &Gd<BattleEntity>) -> bool {
    (*actor).bind().get_stats().bind().is_alive()
}

fn get_entities_belonging_to_faction(
    reference_actor_team: &Team,
    faction: &TargetFaction,
    candidates: &Array<Gd<BattleEntity>>,
) -> Array<Gd<BattleEntity>> {
    let team = get_team_for_faction(reference_actor_team, faction);

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
