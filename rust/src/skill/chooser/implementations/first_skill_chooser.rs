use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::consts::groups::get_skill_factory_node_from_tree;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::chooser::implementations::utils::get_available_skills::get_available_skills;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_factory::SkillFactory;
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
    #[export]
    idle_skill: SkillDefinition,
    idle_skill_node: Option<Gd<Skill>>, // Skill casted when no skills from the skill_pool can be casted
    skill_factory: Option<Gd<SkillFactory>>, // Only used to retrieve the idle_skill
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
        let castable_skills = get_available_skills(&skill_pool, &skill_resource);
        let mut chosen_skill = if castable_skills.len() > 0 {
            castable_skills.at(0)
        } else {
            self.get_idle_skill_node()
        };

        let name = chosen_skill.bind().get_definition();
        let implementation = chosen_skill.bind_mut().get_implementation();
        let priority = chosen_skill.bind().get_priority();
        let target_amount = chosen_skill.bind().get_target_amount();
        let target_faction = chosen_skill.bind().get_target_faction();
        let targets = FirstSkillChooser::get_targets(
            &actor,
            &target_candidates,
            &target_amount,
            &target_faction,
        );

        self.get_skill_chooser()
            .unwrap()
            .signals()
            .skill_chosen()
            .emit(
                name,
                &implementation,
                priority,
                &skill_resource,
                &targets,
                target_amount,
                target_faction,
            );
    }

    fn get_targets(
        actor: &Gd<BattleEntity>,
        target_candidates: &Array<Gd<BattleEntity>>,
        target_amount: &TargetAmount,
        target_faction: &TargetFaction,
    ) -> Array<Gd<BattleEntity>> {
        get_targets_using_mode(
            TargetingMode::FirstAvailable,
            actor,
            target_candidates,
            target_amount,
            target_faction,
        )
    }
    
    fn get_idle_skill_node(&mut self) -> Gd<Skill> {
        if self.idle_skill_node.is_none() {
            let factory = self.get_skill_factory();
            let skill = factory.bind().instance_skill(&self.idle_skill);
            self.idle_skill_node = Some(skill);
            
        }
        self.idle_skill_node.clone().unwrap()
    }
    
    fn get_skill_factory(&mut self) -> Gd<SkillFactory> {
        if self.skill_factory.is_none() {
            let factory = get_skill_factory_node_from_tree(&self.base());
            self.skill_factory = Some(factory);
        }
        self.skill_factory.clone().unwrap()
    }
}
