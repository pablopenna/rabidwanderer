use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::targeting::implementations::first_target_chooser::FirstTargetChooser;
use crate::targeting::mode::TargetingMode;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_chooser::TargetChooser;
use crate::targeting::target_faction::TargetFaction;

pub(crate) fn get_targets_using_mode(
    mode: TargetingMode,
    actor: &Gd<BattleEntity>,
    candidates: &Array<Gd<BattleEntity>>,
    target_amount: &TargetAmount,
    target_faction: &TargetFaction,
) -> Array<Gd<BattleEntity>> {
    match mode {
        TargetingMode::FirstAvailable => FirstTargetChooser::choose_targets(actor, candidates, target_amount, target_faction),
    }
}
