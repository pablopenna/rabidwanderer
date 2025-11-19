use godot::classes::class_macros::private::virtuals::Os::Array;
use godot::obj::Gd;

use crate::battle::entity::entity::BattleEntity;

pub(crate) trait TargetChooser {
    fn choose_targets(
        actor: &Gd<BattleEntity>,
        candidates: &Array<Gd<BattleEntity>>,
    ) -> Array<Gd<BattleEntity>>;
}
