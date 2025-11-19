use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill_definition::SkillDefinition;
use crate::targeting::get_implementation::get_targets_using_mode;
use crate::targeting::mode::TargetingMode;

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
        self.get_skill_chooser()
            .unwrap()
            .signals()
            .choose_skill()
            .connect_other(self, Self::choose);
    }

    fn choose(
        &mut self,
        skill_pool: Gd<SkillContainerModule>,
        skill_resource: Gd<SkillResourceModule>,
        actor: Gd<BattleEntity>,
        target_candidates: Array<Gd<BattleEntity>>,
    ) {
        let mut skill = skill_pool.bind().get_skill_at(0);
        let name = SkillDefinition::from_gstring(skill.bind().get_name());
        let implementation = skill.bind_mut().get_implementation();
        let targets = FirstSkillChooser::get_targets(&actor, &target_candidates);

        self.get_skill_chooser()
            .unwrap()
            .signals()
            .skill_chosen()
            .emit(name, &implementation, &skill_resource, &targets);
    }
    
    fn get_targets(actor: &Gd<BattleEntity>, target_candidates: &Array<Gd<BattleEntity>>) -> Array<Gd<BattleEntity>> {
        get_targets_using_mode(TargetingMode::FirstAvailable, actor, target_candidates)
    }
}
