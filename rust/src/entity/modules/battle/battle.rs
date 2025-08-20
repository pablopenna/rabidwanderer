use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::stats::stats::StatsModule;
use crate::utils::get_first_child_of_type::get_first_child_of_type;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BattleModule {
    #[export]
    battle_entity: OnEditor<Gd<PackedScene>>,
    #[export]
    stats: OnEditor<Gd<StatsModule>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleModule {}

impl BattleModule {
    pub(crate) fn get_battle_module_from_entity(entity: Gd<BoardEntity>) -> Option<Gd<BattleModule>> {
        get_first_child_of_type::<BattleModule>(&entity)
    }

    pub(crate) fn get_battle_entity_instance(&self) -> Gd<BattleEntity> {
        let mut new_instance = self.battle_entity.instantiate_as::<BattleEntity>();
        let real_stats = self.stats.bind().get_stats();

        new_instance.bind_mut().set_stats(real_stats);

        new_instance
    }
}
