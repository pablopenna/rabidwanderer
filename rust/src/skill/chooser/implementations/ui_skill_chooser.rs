use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::global_signals::GlobalSignals;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill::Skill;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct UiSkillChooser {
    base: Base<Node>,
    #[export]
    skill_chooser: OnEditor<Gd<SkillChooser>>,
}

#[godot_api]
impl INode for UiSkillChooser {
    fn ready(&mut self) {
        self.setup();
    }
}

impl UiSkillChooser {
    fn setup(&mut self) {
        self.get_skill_chooser().unwrap().signals().choose_skill().connect_other(self, Self::choose);
    }

    fn choose(&mut self, skill_pool: Gd<SkillContainerModule>, target: Gd<BattleEntity>) {
        
        GlobalSignals::get_singleton().signals().skill_chosen_in_battle_ui().builder()
        .flags(ConnectFlags::ONE_SHOT)
        .connect_other_mut(self, Self::on_skill_chosen_by_ui);
    
        GlobalSignals::get_singleton().signals().show_skills_in_battle_ui().emit(&skill_pool);  
    }

    fn on_skill_chosen_by_ui(&mut self, mut skill: Gd<Skill>) {
        let implementation = skill.bind_mut().get_implementation();

        self.get_skill_chooser().unwrap().signals().skill_chosen().emit(&implementation);
    }
}