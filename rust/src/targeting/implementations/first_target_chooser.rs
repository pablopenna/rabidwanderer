use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::targeting::target_chooser::TargetChooser;

pub(crate) struct FirstTargetChooser {}

impl TargetChooser for FirstTargetChooser {
    fn choose_targets(
        actor: &Gd<BattleEntity>,
        candidates: &Array<Gd<BattleEntity>>,
    ) -> Array<Gd<BattleEntity>> {
        let first_available_target = candidates
            .iter_shared()
            .find(|c| *c != *actor && (*c).bind().get_stats().bind().is_alive());
        let mut targets: Array<Gd<BattleEntity>> = array!();

        if first_available_target.is_some() {
            targets.push(&first_available_target.unwrap());
        }

        targets
    }
}
