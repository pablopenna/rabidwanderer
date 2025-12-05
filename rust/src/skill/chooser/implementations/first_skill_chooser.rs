use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill_definition::SkillDefinition;
use crate::targeting::get_implementation::get_targets_using_mode;
use crate::targeting::mode::TargetingMode;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_faction::TargetFaction;

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
        let mut chosen_skill = skill_pool.bind().get_skill_at(0);
        
        let name = SkillDefinition::from_gstring(chosen_skill.bind().get_name());
        let implementation = chosen_skill.bind_mut().get_implementation();
        let target_amount = TargetAmount::from_gstring(chosen_skill.bind().get_target_amount());
        let target_faction = TargetFaction::from_gstring(chosen_skill.bind().get_target_faction());
        let targets = FirstSkillChooser::get_targets(&actor, &target_candidates, &target_amount, &target_faction);

        self.get_skill_chooser()
            .unwrap()
            .signals()
            .skill_chosen()
            .emit(name, &implementation, &skill_resource, &targets, target_amount, target_faction);
    }

    fn get_targets(
        actor: &Gd<BattleEntity>,
        target_candidates: &Array<Gd<BattleEntity>>,
        target_amount: &TargetAmount,
        target_faction: &TargetFaction,
    ) -> Array<Gd<BattleEntity>> {
        get_targets_using_mode(TargetingMode::FirstAvailable, actor, target_candidates, target_amount, target_faction)
    }
}
