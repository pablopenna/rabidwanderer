use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Action {
    base: Base<Node>,
    actor: Option<Gd<BattleEntity>>,
}

#[godot_api]
impl INode for Action {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            actor: None,
        }
    }
}

#[godot_api]
impl Action {
    #[signal]
    pub(crate) fn turn_ended();
    
    pub(crate) fn get_actor(&self) -> Gd<BattleEntity> {
        self.actor.clone().unwrap()
    }

    pub(crate) fn new(actor: Gd<BattleEntity>) -> Gd<Self>{
        let mut new_turn = Action::new_alloc();
        new_turn.bind_mut().actor = Some(actor);

        new_turn
    }

    pub(crate) fn execute_action(&mut self) {
        godot_print!("Executing action {}", self.base().get_name());
        
        self.actor.clone().unwrap().signals().done_acting().builder()
            // IMPORTANT: If the DEFERRED flag is not used here, a bind_mut() exception will happen
            // I think it is caused by: actor.act() finish almost inmediately, 
            // causing finish_turn to be called then and finding the the mut lock on its own Tile instance 
            // has not been released yet.
            // Update: see https://github.com/godot-rust/gdext/issues/1318
            .flags(ConnectFlags::ONE_SHOT| ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::finish_action);

        // self.actor.clone().unwrap().bind_mut().act();
        BattleEntity::act_with_skill(self.actor.clone().unwrap());
    }

    pub(crate) fn finish_action(&mut self) {
        godot_print!("Finishing action {}", self.base().get_name());
        self.signals().turn_ended().emit();
        // self.base_mut().call_deferred("queue_free", &[]);
    }
}
