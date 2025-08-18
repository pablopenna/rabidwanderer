use godot::classes::*;
use godot::prelude::*;

use crate::battle::setup::BattleSetup;
use crate::consts::groups::get_battle_setup_node_from_tree;
use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BattleTriggerModule {
    entity: Option<Gd<BoardEntity>>,
    battle_setup: Option<Gd<BattleSetup>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleTriggerModule {
    fn ready(&mut self) {
        let parent = self.base_mut().to_godot().upcast::<Node>().get_parent().unwrap();
        self.entity = Some(parent.cast::<BoardEntity>());

        self.entity.clone().unwrap().signals()
            .moved_board_tile()
            .connect_other(self, Self::trigger_battle_setup);
    }
}

impl BattleTriggerModule {
    fn trigger_battle_setup(&mut self) {
        let coord = self.entity.clone().unwrap();
        let coord = coord.bind();
        let coord = coord.get_coordinates();
        self.get_battle_setup_node().bind_mut().setup_combat_for_tile(coord);
    }

    fn get_battle_setup_node(&mut self) -> Gd<BattleSetup> {
        if self.battle_setup.is_none() {
            self.battle_setup = Some(get_battle_setup_node_from_tree(self.base().clone().upcast::<Node>()));
        }

        self.battle_setup.clone().unwrap()
    }
}
