use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BattleModule {
    #[export]
    battle_entity: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleModule {}

impl BattleModule {
    pub(crate) fn get_battle_module_from_entity(entity: Gd<BoardEntity>) -> Option<Gd<BattleModule>> {
        BoardEntity::get_first_child_of_type::<BattleModule>(&entity)
    }

    fn get_battle_entity_instance(&mut self) -> Gd<BattleEntity> {
        self.battle_entity.instantiate_as::<BattleEntity>()
    }
}
