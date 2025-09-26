use godot::obj::Gd;

use crate::{battle::entity::entity::BattleEntity, entity::modules::skill::skill_container::SkillContainerModule, skill::skill_definition::SkillDefinition};

pub(crate) trait SkillChooser {
    fn choose(&mut self, skill_pool: &Gd<SkillContainerModule>, target: &Gd<BattleEntity>) -> SkillDefinition;
}