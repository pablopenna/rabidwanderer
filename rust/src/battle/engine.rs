use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::turns::action::Action;
use crate::battle::turns::turn_handler::TurnHandler;
use crate::consts::groups::BATTLE_ENGINE_GROUP;
use crate::global_signals::GlobalSignals;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleEngine {
    base: Base<Node>,
    #[export]
    turn_handler: OnEditor<Gd<TurnHandler>>,
    #[export]
    battle_entity_container: OnEditor<Gd<BattleEntityContainer>>,
    current_action: Option<Gd<Action>>,
}

#[godot_api]
impl INode for BattleEngine {
    fn init(base: Base<Node>) -> Self {
        Self {
            turn_handler: OnEditor::default(),
            battle_entity_container: OnEditor::default(),
            current_action: None,
            base
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group(BATTLE_ENGINE_GROUP);

        GlobalSignals::get_singleton().signals().battle_ui_shown().connect_other(self, Self::start_battle);
    }
}


#[godot_api]
impl BattleEngine {

    #[func]
    pub(crate) fn start_battle(&mut self) {
        if self.battle_entity_container.bind().get_entity_count() < 2 {
            godot_print!("Not enough entities to battle!");
            return;
        }
        GlobalSignals::get_singleton().signals().battle_started().emit();
        
        self.update_battle();
    }

    fn update_battle(&mut self) {
        if !self.turn_handler.bind_mut().are_there_actions_left_in_turn() {
            godot_print!("new turn!");
            self.turn_handler.bind_mut().generate_new_turn(
                Vec::from_godot(self.battle_entity_container.bind().get_all_entities())
            );
        }

        if self.current_action.is_none() {
            godot_print!("Getting new action!");
            let action = self.turn_handler.bind_mut().get_next_action_from_turn();
            self.current_action = Some(action.unwrap());
        }

        let mut current_action = self.current_action.clone().unwrap();
        current_action.signals().turn_ended().builder()
            .flags(ConnectFlags::ONE_SHOT | ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::on_current_action_done);
        current_action.bind_mut().execute_action();
    }

    fn on_current_action_done(&mut self) {
        let mut last_action = self.current_action.clone().unwrap();
        last_action.signals().tree_exited().builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect_other_mut(self, Self::after_action_cleanup);
        last_action.queue_free();
        godot_print!("Cleaning last action...");
    }

    fn after_action_cleanup(&mut self) {
        godot_print!("Last turn cleaned");
        self.current_action = None;
        let alive_entities = self.battle_entity_container.clone().bind().get_alive_entities();
        if alive_entities.len() <= 1 {
            godot_print!("Only one is left standing...");
            self.on_battle_end();
            return;
        }
        
        self.update_battle();
    }

    fn on_battle_end(&mut self) {
        GlobalSignals::get_singleton().signals().battle_finished().emit();
    }
}
