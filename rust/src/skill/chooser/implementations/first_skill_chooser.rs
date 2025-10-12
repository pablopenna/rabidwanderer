use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct FirstSkillChooser {
    base: Base<Node>,
    #[export]
    skill_chooser: OnEditor<Gd<SkillChooser>>,
}

#[godot_api]
impl INode for FirstSkillChooser {
    fn ready(&mut self) {
        self.setup();
    }
}

impl FirstSkillChooser {
    fn setup(&mut self) {
        self.get_skill_chooser().unwrap().signals().choose_skill().connect_other(self, Self::choose);
    }

    fn choose(&mut self, skill_pool: Gd<SkillContainerModule>, target: Gd<BattleEntity>) {
        let mut skill = skill_pool.bind().get_skill_at(0);
        let name = SkillDefinition::from_gstring(skill.bind().get_name());
        let implementation = skill.bind_mut().get_implementation();

        self.get_skill_chooser().unwrap().signals().skill_chosen().emit(name, &implementation);
    }
}