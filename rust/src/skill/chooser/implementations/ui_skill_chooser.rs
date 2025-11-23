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
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct UiSkillChooser {
    base: Base<Node>,
    #[export]
    skill_chooser: OnEditor<Gd<SkillChooser>>,
    actor: Option<Gd<BattleEntity>>,
    status: Status,
}

// Attributes to store temporary data as I could not pass it along among methods with signals in between
struct Status {
    _skill_definition: Option<SkillDefinition>,
    _skill_implementation: Option<DynGd<Node, dyn SkillImplementation>>,
    _skill_resource: Option<Gd<SkillResourceModule>>,
    _targets: Option<Array<Gd<BattleEntity>>>,
}

#[godot_api]
impl INode for UiSkillChooser {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            skill_chooser: OnEditor::default(),
            actor: None,
            status: Status {
                _skill_definition: None,
                _skill_implementation: None,
                _skill_resource: None,
                _targets: None,
            },
        }
    }

    fn ready(&mut self) {
        self.setup();
    }
}

// 1. Choose Skill via UI
// 2. Choose target
impl UiSkillChooser {
    fn setup(&mut self) {
        self.get_skill_chooser()
            .unwrap()
            .signals()
            .choose_skill()
            .connect_other(self, Self::choose_skill_via_ui);
    }

    fn clear_status(&mut self) {
        self.status = Status {
            _skill_definition: None,
            _skill_implementation: None,
            _skill_resource: None,
            _targets: None,
        }
    }

    fn choose_skill_via_ui(
        &mut self,
        skill_pool: Gd<SkillContainerModule>,
        skill_resource: Gd<SkillResourceModule>,
        actor: Gd<BattleEntity>,
        _target_candidates: Array<Gd<BattleEntity>>, // TODO: use when TargetingType == ALL_ENEMIES, ALL, etc.
    ) {
        self.actor = Some(actor);
        self.clear_status();

        godot_print!("Choosing skill via UI...");

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
        self.status._skill_definition =
            Some(SkillDefinition::from_gstring(skill.bind().get_name()));
        self.status._skill_implementation = Some(skill.bind_mut().get_implementation());
        self.status._skill_resource = Some(skill_resource);

        godot_print!("Skill chosen via UI!");

        // TODO: Trigger code that gates the UI to show targeting frames. Right now is always enabled.
        self.choose_target_via_ui();
    }

    fn choose_target_via_ui(&mut self) {
        godot_print!("Choosing target via UI...");

        GlobalSignals::get_singleton()
            .signals()
            .entity_targeted_via_ui()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(self, Self::on_target_chosen_by_ui);
    }

    fn on_target_chosen_by_ui(&mut self, target: Gd<BattleEntity>) {
        self.status._targets = Some(array!(&target));

        godot_print!("Target chosen via UI!");

        self.finish_choosing();
    }

    fn finish_choosing(&mut self) {
        godot_print!("Done choosing.");

        self.get_skill_chooser()
            .unwrap()
            .signals()
            .skill_chosen()
            .emit(
                self.status._skill_definition.clone().unwrap(),
                &self.status._skill_implementation.clone().unwrap(),
                &self.status._skill_resource.clone().unwrap(),
                &self.status._targets.clone().unwrap(),
            );
    }
}
