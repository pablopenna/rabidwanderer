use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::ui::hp_bar::HpBar;
use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::stats::stats::StatsModule;
use crate::stats::real::RealStats;
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

        add_stats_to_entity(real_stats.clone(), &mut new_instance);
        add_hp_bar(real_stats.clone(), &mut new_instance);

        new_instance
    }

}

fn add_stats_to_entity(stats: Gd<RealStats>, entity: &mut Gd<BattleEntity>) {
    entity.bind_mut().set_stats(stats);
}

fn add_hp_bar(stats_to_track: Gd<RealStats>, entity: &mut Gd<BattleEntity>) {
    let mut hp_bar: Gd<HpBar> = HpBar::new_alloc();
    hp_bar.bind_mut().setup(stats_to_track);

    entity.add_child(&hp_bar);
}
