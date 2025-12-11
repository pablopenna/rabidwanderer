use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::turns::action::Action;
use crate::stats::stat::Stat;

// This class represents a turn, where each battler performs at least one action.
// Actions are child nodes of this one.
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct TurnHandler {
    base: Base<Node>,
}

#[godot_api]
impl INode for TurnHandler {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl TurnHandler {
    #[signal]
    pub(crate) fn actions_ready();

    pub(crate) fn get_next_action_from_turn(&mut self) -> Option<Gd<Action>> {
        let mut result = None;
        let child_count = self.base_mut().get_child_count();
        for i in 0..child_count {
            let child = self.base_mut().get_child(i);
            if child.is_none() {
                continue;
            }
            let child = child.unwrap();
            let child = child.try_cast::<Action>();
            if child.is_ok() {
                result = child.ok();
            }
        }

        result
    }

    pub(crate) fn add_action_to_turn(&mut self, action: Gd<Action>) {
        self.base_mut().add_child(&action);
    }

    pub(crate) fn are_there_actions_left_in_turn(&self) -> bool {
        self.base().get_child_count() > 0
    }

    /** Should wait for the actions_ready signal to be emitted before actions are processed as skills might not be chosen */
    pub(crate) fn generate_new_turn(&mut self, mut battlers: Array<Gd<BattleEntity>>) {
        godot_print!("Generating actions for new turn");
        battlers.sort_unstable_by(|a, b| {
            let a_speed = a.bind().get_stats().bind().get_stat(Stat::Speed);
            let b_speed = b.bind().get_stats().bind().get_stat(Stat::Speed);
            a_speed.cmp(&b_speed)
        });

        battlers.iter_shared().for_each(|battler| {
            let new_action = Action::new(battler.clone());
            new_action
                .signals()
                .action_ready()
                .builder()
                .flags(ConnectFlags::ONE_SHOT)
                .connect_other_mut(self, Self::emit_ready_signal_if_all_actions_ready);
            self.add_action_to_turn(new_action);
        });
    }

    fn emit_ready_signal_if_all_actions_ready(&mut self) {
        let are_all_actions_ready = self
            .base()
            .get_children()
            .iter_shared()
            .map(|c| c.cast::<Action>())
            .all(|action| action.bind().get_skill().is_some());

        if are_all_actions_ready {
            // TODO: sort actions by priority and then speed.
            self.signals().actions_ready().emit();
        }
    }

    pub(crate) fn remove_all_actions_from_turn(&mut self) {
        let children = self.base_mut().get_children();
        children
            .iter_shared()
            .for_each(|child| self.base_mut().remove_child(&child));
    }
}
