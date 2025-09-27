use godot::{classes::Node, obj::{DynGd, Gd}};

use crate::{battle::entity::entity::BattleEntity, entity::modules::skill::skill_container::SkillContainerModule, skill::skill_implementation::SkillImplementation};

pub(crate) trait SkillChooser {
    fn choose(&mut self, skill_pool: &Gd<SkillContainerModule>, target: &Gd<BattleEntity>) -> DynGd<Node, dyn SkillImplementation>;
}