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
            TargetFaction::get_entities_belonging_to_faction(&actor_team, target_faction, candidates);
        
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
