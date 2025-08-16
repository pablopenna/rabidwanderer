use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::BattleEntity;
use crate::battle::turns::turn::Turn;
use crate::battle::turns::turns_handler::TurnsHandler;
use crate::consts::groups::BATTLE_ENGINE_GROUP;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleEngine {
    base: Base<Node>,
    #[export]
    player: Option<Gd<BattleEntity>>,
    #[export]
    enemy: Option<Gd<BattleEntity>>,
    turns_handler: Gd<TurnsHandler>,
    battle_is_done: bool,
    current_turn: Option<Gd<Turn>>,
}

#[godot_api]
impl INode for BattleEngine {
    fn init(base: Base<Node>) -> Self {
        Self {
            // battling_entities: OnReady::manual(),
            player: None,
            enemy: None,
            turns_handler: TurnsHandler::new_alloc(),
            battle_is_done: true,
            current_turn: None,
            base
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group(BATTLE_ENGINE_GROUP);

        let turns_handler_node = self.turns_handler.clone().upcast::<Node>();
        self.base_mut().add_child(&turns_handler_node);
    }
}


#[godot_api]
impl BattleEngine {
    #[func]
    pub(crate) fn start_battle(&mut self) {
        if self.player.is_none() || self.enemy.is_none() {
            godot_error!("Missing battle entities!");
            return;
        }
        
        self.battle_is_done = false;
        self.update_battle();
    }

    fn update_battle(&mut self) {
        if self.battle_is_done {
            return;
        }

        if !self.turns_handler.bind_mut().are_there_turns_left() {
            self.generate_new_turns(vec!(self.player.clone().unwrap(), self.enemy.clone().unwrap()));
        }

        if self.current_turn.is_none() {
            godot_print!("Getting new turn!");
            let turn = self.turns_handler.bind_mut().get_next_turn_from_pool();
            self.current_turn = Some(turn.unwrap());
        }

        let mut current_turn = self.current_turn.clone().unwrap();
        current_turn.signals().turn_ended().builder()
            .flags(ConnectFlags::ONE_SHOT| ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::on_current_turn_done);
        current_turn.bind_mut().execute_turn();
    }

    fn generate_new_turns(&mut self, mut battlers: Vec<Gd<BattleEntity>>) {
        godot_print!("Generating new turns");
        battlers.sort_by(|a, b| {
            let a_speed = a.bind().get_stats().unwrap().bind().get_speed();
            let b_speed = b.bind().get_stats().unwrap().bind().get_speed();
            a_speed.cmp(&b_speed)
        });

        battlers.iter().for_each(|battler| {
            let new_turn = Turn::new(battler.clone());
            self.turns_handler.bind_mut().add_turn_to_pool(new_turn);
        });
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
        if !self.player.clone().unwrap().bind().get_stats().unwrap().bind().is_alive() || !self.enemy.clone().unwrap().bind().get_stats().unwrap().bind().is_alive() {
            godot_print!("One is dead...");
            self.battle_is_done = true;
        }
        self.update_battle();
    }
}


