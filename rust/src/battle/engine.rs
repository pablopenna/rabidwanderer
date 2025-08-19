use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::turns::turn::Turn;
use crate::battle::turns::turns_handler::TurnsHandler;
use crate::consts::groups::BATTLE_ENGINE_GROUP;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleEngine {
    base: Base<Node>,
    #[export]
    turns_handler: OnEditor<Gd<TurnsHandler>>,
    #[export]
    battle_entity_container: OnEditor<Gd<BattleEntityContainer>>,
    current_turn: Option<Gd<Turn>>,
}

#[godot_api]
impl INode for BattleEngine {
    fn init(base: Base<Node>) -> Self {
        Self {
            turns_handler: OnEditor::default(),
            battle_entity_container: OnEditor::default(),
            current_turn: None,
            base
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group(BATTLE_ENGINE_GROUP);
    }
}


#[godot_api]
impl BattleEngine {

    #[signal]
    pub(crate) fn battle_ended();

    #[func]
    pub(crate) fn start_battle(&mut self) {
        if self.battle_entity_container.bind().get_entity_count() < 2 {
            godot_print!("Not enough entities to battle!");
            return;
        }
        
        self.update_battle();
    }

    fn update_battle(&mut self) {
        if !self.turns_handler.bind_mut().are_there_turns_left() {
            self.turns_handler.bind_mut().generate_new_turns(
                Vec::from_godot(self.battle_entity_container.bind().get_all_entities())
            );
        }

        if self.current_turn.is_none() {
            godot_print!("Getting new turn!");
            let turn = self.turns_handler.bind_mut().get_next_turn_from_pool();
            self.current_turn = Some(turn.unwrap());
        }

        let mut current_turn = self.current_turn.clone().unwrap();
        current_turn.signals().turn_ended().builder()
            .flags(ConnectFlags::ONE_SHOT | ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::on_current_turn_done);
        current_turn.bind_mut().execute_turn();
    }

    fn on_current_turn_done(&mut self) {
        let mut last_turn = self.current_turn.clone().unwrap();
        last_turn.signals().tree_exited().builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(self, Self::after_turn_cleanup);
        last_turn.queue_free();
        godot_print!("Cleaning last turn...");
    }

    fn after_turn_cleanup(&mut self) {
        godot_print!("Last turn cleaned");
        self.current_turn = None;
        if self.battle_entity_container.clone().bind().get_alive_entities().len() <= 1 {
            godot_print!("Only one is left standing...");
            self.on_battle_end();
            return;
        }
        
        self.update_battle();
    }

    fn on_battle_end(&mut self) {
        self.signals().battle_ended().emit();
    }
}
