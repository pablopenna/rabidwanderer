use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::consts::groups::get_skill_factory_node_from_tree;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::global_signals::GlobalSignals;
use crate::skill::chooser::implementations::utils::can_cast_at_least_one_skill::can_cast_at_least_one_skill;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_factory::SkillFactory;
use crate::skill::skill_implementation::SkillImplementation;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_faction::TargetFaction;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct UiSkillChooser {
    base: Base<Node>,
    #[export]
    skill_chooser: OnEditor<Gd<SkillChooser>>,
    actor: Option<Gd<BattleEntity>>,
    status: Status,
    #[export]
    idle_skill: SkillDefinition,
    idle_skill_node: Option<Gd<Skill>>, // Skill casted when no skills from the skill_pool can be casted
    skill_factory: Option<Gd<SkillFactory>>, // Only used to retrieve the idle_skill
}

// Attributes to store temporary data as I could not pass it along among methods with signals in between
struct Status {
    _skill_definition: Option<SkillDefinition>,
    _skill_implementation: Option<DynGd<Node, dyn SkillImplementation>>,
    _skill_priority: Option<i32>,
    _skill_resource: Option<Gd<SkillResourceModule>>,
    _skill_target_faction: Option<TargetFaction>,
    _skill_target_amount: Option<TargetAmount>,
    _target_candidates: Option<Array<Gd<BattleEntity>>>,
    _targets_chosen: Option<Array<Gd<BattleEntity>>>,
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
                _skill_priority: None,
                _skill_resource: None,
                _skill_target_faction: None,
                _skill_target_amount: None,
                _target_candidates: None,
                _targets_chosen: None,
            },
            idle_skill: SkillDefinition::Idle,
            idle_skill_node: None,
            skill_factory: None,
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
            _skill_priority: None,
            _skill_resource: None,
            _skill_target_faction: None,
            _skill_target_amount: None,
            _target_candidates: None,
            _targets_chosen: None,
        }
    }

    fn choose_skill_via_ui(
        &mut self,
        skill_pool: Gd<SkillContainerModule>,
        skill_resource: Gd<SkillResourceModule>,
        actor: Gd<BattleEntity>,
        target_candidates: Array<Gd<BattleEntity>>, // TODO: use when TargetingType == ALL_ENEMIES, ALL, etc.
    ) {
        self.actor = Some(actor);
        self.clear_status();
        
        godot_print!("Choosing skill via UI...");
        
        if !can_cast_at_least_one_skill(&skill_pool, &skill_resource) {
            godot_print!("No available skill!");
            let idle_skill = self.get_idle_skill_node();
            self.on_skill_chosen_by_ui(idle_skill, skill_resource.clone());
            return;
        }

        GlobalSignals::get_singleton()
            .signals()
            .skill_chosen_in_battle_ui()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(self, Self::on_skill_chosen_by_ui);

        self.status._target_candidates = Some(target_candidates.clone());
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
            Some(skill.bind().get_definition());
        self.status._skill_implementation = Some(skill.bind_mut().get_implementation());
        self.status._skill_priority = Some(skill.bind().get_priority());
        self.status._skill_resource = Some(skill_resource);
        self.status._skill_target_amount =
            Some(skill.bind().get_target_amount());
        self.status._skill_target_faction = Some(
            skill.bind().get_target_faction(),
        );

        // filter candidates based in faction as after choosing not all candidates are valid
        let actor_team = self.actor.clone().unwrap().bind().get_entity_team();
        let target_faction = self.status._skill_target_faction.clone().unwrap();
        let target_amount = self.status._skill_target_amount.clone().unwrap();

        self.status._target_candidates = Some(TargetFaction::get_entities_belonging_to_faction(
            &actor_team,
            &target_faction,
            & self.status._target_candidates.clone().unwrap(),
        ));
        let candidates = self.status._target_candidates.clone().unwrap();

        godot_print!("Skill chosen via UI!");

        if target_amount == TargetAmount::All || candidates.len() == 1 {
            self.status._targets_chosen = self.status._target_candidates.clone();
            self.finish_choosing();
            return;
        }

        self.choose_target_via_ui(&self.status._target_candidates.clone().unwrap());
    }

    fn choose_target_via_ui(&mut self, candidates: &Array<Gd<BattleEntity>>) {
        godot_print!("Choosing target via UI...");

        GlobalSignals::get_singleton()
            .signals()
            .targets_chosen_via_ui()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(self, Self::on_target_chosen_by_ui);

        GlobalSignals::get_singleton()
            .signals()
            .choose_target_via_ui()
            .emit(candidates)
    }

    fn on_target_chosen_by_ui(&mut self, targets: Array<Gd<BattleEntity>>) {
        self.status._targets_chosen = Some(targets);

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
                self.status._skill_priority.clone().unwrap(),
                &self.status._skill_resource.clone().unwrap(),
                &self.status._targets_chosen.clone().unwrap(),
                self.status._skill_target_amount.clone().unwrap(),
                self.status._skill_target_faction.clone().unwrap(),
            );
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
