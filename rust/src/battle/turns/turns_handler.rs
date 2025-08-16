use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::turns::turn::Turn;

// Turns are child nodes of this one
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct TurnsHandler {
    base: Base<Node>,
}

#[godot_api]
impl INode for TurnsHandler {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl TurnsHandler {
    
    pub(crate) fn get_next_turn_from_pool(&mut self) -> Option<Gd<Turn>> {
        let mut result = None;
        let child_count = self.base_mut().get_child_count();
        for i in 0..child_count {
            let child = self.base_mut().get_child(i);
            if child.is_none() {
                continue;
            }
            let child = child.unwrap();
            let child = child.try_cast::<Turn>();
            if child.is_ok() {
                result = child.ok();
            }
        }
        
        result
    }

    pub(crate) fn add_turn_to_pool(&mut self, turn: Gd<Turn>) {
        self.base_mut().add_child(&turn);
    }

    pub(crate) fn are_there_turns_left(& self) -> bool {
        self.base().get_child_count() > 0
    }

    pub(crate) fn generate_new_turns(&mut self, mut battlers: Vec<Gd<BattleEntity>>) {
        godot_print!("Generating new turns");
        battlers.sort_by(|a, b| {
            let a_speed = a.bind().get_stats().unwrap().bind().get_speed();
            let b_speed = b.bind().get_stats().unwrap().bind().get_speed();
            a_speed.cmp(&b_speed)
        });

        battlers.iter().for_each(|battler| {
            let new_turn = Turn::new(battler.clone());
            self.add_turn_to_pool(new_turn);
        });
    }
}