use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::global_signals::GlobalSignals;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;
use crate::targeting::get_implementation::get_targets_using_mode;
use crate::targeting::mode::TargetingMode;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct UiSkillChooser {
    base: Base<Node>,
    #[export]
    skill_chooser: OnEditor<Gd<SkillChooser>>,
    actor: Option<Gd<BattleEntity>>,
    target_candidates: Array<Gd<BattleEntity>>,
}

#[godot_api]
impl INode for UiSkillChooser {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            skill_chooser: OnEditor::default(),
            actor: None,
            target_candidates: array!(),
        }
    }
    
    fn ready(&mut self) {
        self.setup();
    }
}

impl UiSkillChooser {
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
        self.actor = Some(actor);
        self.target_candidates.clone_from(&target_candidates);
        
        GlobalSignals::get_singleton()
            .signals()
            .skill_chosen_in_battle_ui()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(self, Self::on_skill_chosen_by_ui);

        GlobalSignals::get_singleton()
            .signals()
            .show_skills_in_battle_ui()
            .emit(&skill_pool, &skill_resource);
    }

    fn on_skill_chosen_by_ui(
        &mut self,
        mut skill: Gd<Skill>,
        skill_resource: Gd<SkillResourceModule>,
    ) {
        let name = SkillDefinition::from_gstring(skill.bind().get_name());
        let implementation = skill.bind_mut().get_implementation();
        let targets = UiSkillChooser::get_targets(&self.actor.clone().unwrap(), &self.target_candidates);

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
