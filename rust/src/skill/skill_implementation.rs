use godot::{classes::class_macros::private::virtuals::Os::Array, obj::Gd};

use crate::battle::entity::entity::BattleEntity;

// https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.DynGd.html
pub(crate) trait SkillImplementation {
    fn cast(&mut self, user: Gd<BattleEntity>, targets: &Array<Gd<BattleEntity>>);
}