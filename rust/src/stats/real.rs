use godot::classes::*;
use godot::prelude::*;

use crate::stats::base::BaseStats;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct RealStats {
    base: Base<Node>,
    #[export]
    base_stats: OnEditor<Gd<BaseStats>>,
    current_hp: u16,
}

#[godot_api]
impl INode for RealStats {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            base_stats: OnEditor::default(),
            current_hp: 0,
        }
    }

    fn ready(&mut self) {
        self.current_hp = self.base_stats.bind().get_max_hp();
    }
}

#[godot_api]
impl RealStats {

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