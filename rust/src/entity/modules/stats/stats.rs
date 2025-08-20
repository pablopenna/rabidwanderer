use godot::classes::*;
use godot::prelude::*;

use crate::stats::base::BaseStats;
use crate::stats::real::RealStats;

/*
* The purpose of this module is to create a RealStats Resource
* from a BaseStats one. This way, we can have one BaseStats per
* entity type (e.g. Player or Wolf) but every single entity in the game
* will have a dedicated RealStats resource. This is important because RealStats
* should not be shared across entities as they contain individual information
* as current HP.
*
* Thanks to RealStats being a Resource, the BoardEntity can setup the BattleEntity
* with their same RealStats and share the data without extra work (it is passed by reference
* and I do not need to reparent it every time). This way, the Player stats (HP) is retained between
* battles.
*/

#[derive(GodotClass)]
#[class(base=Node)]
pub struct StatsModule {
    #[export]
    base_stats: OnEditor<Gd<BaseStats>>,
    real_stats: OnReady<Gd<RealStats>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for StatsModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            base_stats: OnEditor::default(),
            real_stats: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.real_stats.init(RealStats::new(self.base_stats.get_property().unwrap()))
    }
}

impl StatsModule {
    pub(crate) fn get_stats(&self) -> Gd<RealStats> {
        self.real_stats.get_property()
    }
}
