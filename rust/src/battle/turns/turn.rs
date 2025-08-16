use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::BattleEntity;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Turn {
    base: Base<Node>,
    actor: Option<Gd<BattleEntity>>,
    is_in_progress: bool,
}

#[godot_api]
impl INode for Turn {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            actor: None,
            is_in_progress: false,
        }
    }
}

#[godot_api]
impl Turn {
    #[signal]
    pub(crate) fn turn_ended();

    pub(crate) fn new(actor: Gd<BattleEntity>) -> Gd<Self>{
        let mut new_turn = Turn::new_alloc();
        new_turn.bind_mut().actor = Some(actor);

        new_turn
    }

    pub(crate) fn is_in_progress(&self) -> bool {
        self.is_in_progress
    }

    pub(crate) fn execute_turn(&mut self) {
        godot_print!("Executing turn {}", self.base().get_name());
        self.is_in_progress = true;
        
        self.actor.clone().unwrap().signals().done_acting().builder()
            // IMPORTANT: If the DEFERRED flag is not used here, a bind_mut() exception will happen
            // I think it is caused by: actor.act() finish almost inmediately, 
            // causing finish_turn to be called then and finding the the mut lock on its own Tile instance 
            // has not been released yet.
            .flags(ConnectFlags::ONE_SHOT| ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::finish_turn);
        // self.actor.clone().unwrap().signals().done_acting().connect_other(self, Self::finish_turn);

        self.actor.clone().unwrap().bind_mut().act();

        // self.finish_turn();
    }

    pub(crate) fn finish_turn(&mut self) {
        godot_print!("Finishing turn {}", self.base().get_name());
        self.signals().turn_ended().emit();
        // self.base_mut().call_deferred("queue_free", &[]);
    }
}
