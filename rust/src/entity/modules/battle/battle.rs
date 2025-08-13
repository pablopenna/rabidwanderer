use godot::classes::*;
use godot::prelude::*;

use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BattleModule {
    entity: Option<Gd<BoardEntity>>,
    #[export]
    stats: u32,
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleModule {
    fn ready(&mut self) {
        let parent = self.base_mut().to_godot().upcast::<Node>().get_parent().unwrap();
        self.entity = Some(parent.cast::<BoardEntity>());

        self.entity.clone().unwrap().signals().on_interact().connect_other(self, Self::on_interact_with);
    }
}

impl BattleModule {
    fn on_interact_with(&mut self, _this_entity: Gd<BoardEntity>, other_entity: Gd<BoardEntity>) {
        godot_print!("interacting...");
        let other_battle_module = BattleModule::get_battle_module_from_entity(other_entity.clone());
        if other_battle_module.is_none() {
            return;
        }
        let other_stats = other_battle_module.unwrap().bind_mut().get_stats();

        // mock logic
        let winner: StringName = if self.stats > other_stats { other_entity.clone().get_name() } else { _this_entity.clone().get_name() };
        godot_print!("{} wins", winner);
    }

    fn get_battle_module_from_entity(entity: Gd<BoardEntity>) -> Option<Gd<BattleModule>> {
        BoardEntity::get_first_child_of_type::<BattleModule>(&entity)
    }
}
