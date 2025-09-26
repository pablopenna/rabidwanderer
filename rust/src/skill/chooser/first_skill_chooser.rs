use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct FirstSkillChooser {
    base: Base<Node>
}

#[godot_dyn]
impl SkillChooser for FirstSkillChooser {
    fn choose(&mut self, skill_pool: &Gd<SkillContainerModule>, _target: &Gd<BattleEntity>) -> SkillDefinition {
        let skill = skill_pool.bind().get_skill_at(0);
        let definition = SkillDefinition::from_gstring(skill.bind().get_name());

        definition
    }
}