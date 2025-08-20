use godot::classes::*;
use godot::prelude::*;

use crate::stats::base::BaseStats;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub(crate) struct RealStats {
    base: Base<Resource>,
    #[export]
    base_stats: OnEditor<Gd<BaseStats>>,
    current_hp: u16,
}

#[godot_api]
impl RealStats {

    pub(crate) fn new(base_stats: Gd<BaseStats>) -> Gd<Self> {
        let mut new_stats = Self::new_gd();
        new_stats.bind_mut().current_hp = base_stats.bind().get_max_hp();
        new_stats.bind_mut().base_stats.init(base_stats);
        
        new_stats
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub(crate) fn get_current_hp(&self) -> u16 {
        self.current_hp
    }

    pub(crate) fn set_current_hp(&mut self, hp: u16) {
        self.current_hp = hp;
    }

    pub(crate) fn get_attack(&self) -> u16 {
        self.base_stats.bind().get_attack()
    }

    pub(crate) fn get_speed(&self) -> u16 {
        self.base_stats.bind().get_speed()
    }
}