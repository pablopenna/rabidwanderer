use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::targeting::implementations::first_target_chooser::FirstTargetChooser;
use crate::targeting::mode::TargetingMode;
use crate::targeting::target_chooser::TargetChooser;

pub(crate) fn get_targets_using_mode(
    mode: TargetingMode,
    actor: &Gd<BattleEntity>,
    candidates: &Array<Gd<BattleEntity>>,
) -> Array<Gd<BattleEntity>> {
    match mode {
        TargetingMode::FirstAvailable => FirstTargetChooser::choose_targets(actor, candidates),
    }
}
