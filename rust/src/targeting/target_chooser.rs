use godot::classes::class_macros::private::virtuals::Os::Array;
use godot::obj::Gd;

use crate::battle::entity::entity::BattleEntity;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_faction::TargetFaction;

pub(crate) trait TargetChooser {
    fn choose_targets(
        actor: &Gd<BattleEntity>,
        candidates: &Array<Gd<BattleEntity>>,
        target_amount: &TargetAmount,
        target_faction: &TargetFaction,
    ) -> Array<Gd<BattleEntity>>;
}
