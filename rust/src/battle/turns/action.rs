use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Action {
    base: Base<Node>,
    actor: Option<Gd<BattleEntity>>,
    skill_to_cast: Option<SkillDefinition>,
    skill_targets: Array<Gd<BattleEntity>>,
}

#[godot_api]
impl INode for Action {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            actor: None,
            skill_to_cast: None,
            skill_targets: Array::default(),
        }
    }
}

#[godot_api]
impl Action {
    #[signal]
    pub(crate) fn action_ready();

    #[signal]
    pub(crate) fn action_ended();

    pub(crate) fn get_actor(&self) -> Gd<BattleEntity> {
        self.actor.clone().unwrap()
    }

    pub(crate) fn get_skill(&self) -> Option<SkillDefinition> {
        self.skill_to_cast.clone()
    }

    pub(crate) fn new(actor: Gd<BattleEntity>) -> Gd<Self> {
        let mut new_turn = Action::new_alloc();
        new_turn.bind_mut().actor = Some(actor.clone());
        
        actor
            .signals()
            .skill_chosen()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(&new_turn, Self::set_skill_and_targets);
        
        BattleEntity::choose_skill(actor);

        new_turn
    }

    fn set_skill_and_targets(&mut self, skill: SkillDefinition, targets: Array<Gd<BattleEntity>>) {
        self.skill_to_cast = Some(skill);
        self.skill_targets.clone_from(&targets);
        
        self.signals().action_ready().emit();
    }

    pub(crate) fn execute_action(&mut self) {
        godot_print!("Executing action {}", self.base().get_name());

        self.actor
            .clone()
            .unwrap()
            .signals()
            .done_acting()
            .builder()
            // IMPORTANT: If the DEFERRED flag is not used here, a bind_mut() exception will happen
            // I think it is caused by: actor.act() finish almost inmediately,
            // causing finish_turn to be called then and finding the the mut lock on its own Tile instance
            // has not been released yet.
            // Update: see https://github.com/godot-rust/gdext/issues/1318
            .flags(ConnectFlags::ONE_SHOT | ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::finish_action);

        // self.actor.clone().unwrap().bind_mut().act();
        // BattleEntity::choose_skill(self.actor.clone().unwrap());
        let skill = self.skill_to_cast.clone().unwrap();
        let targets = self.skill_targets.clone();
        BattleEntity::cast_skill(self.actor.clone().unwrap(), skill, targets);
    }

    pub(crate) fn finish_action(&mut self) {
        godot_print!("Finishing action {}", self.base().get_name());
        self.signals().action_ended().emit();
        // self.base_mut().call_deferred("queue_free", &[]);
    }
}
