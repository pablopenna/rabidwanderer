use godot::classes::*;
use godot::prelude::*;

use rand::Rng;

use crate::stats::base::BaseStats;

const DAMAGE_VARIATION_RATIO: f32 = 0.1;

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
    #[signal]
    pub(crate) fn no_hp_left();
    #[signal]
    pub(crate) fn hp_changed(new_hp: u16);

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
        self.signals().hp_changed().emit(hp);
        if self.current_hp <= 0 {
            self.signals().no_hp_left().emit();
        }
    }

    pub(crate) fn get_max_hp(&self) -> u16 {
        self.base_stats.bind().get_max_hp()
    }

    pub(crate) fn get_attack(&self) -> u16 {
        self.base_stats.bind().get_attack()
    }

    pub(crate) fn calculate_attack_damage(&self) -> u16 {
        let base_damage = self.get_attack();
        let mut rng = rand::rng();
        let variation: f32 = base_damage as f32 * DAMAGE_VARIATION_RATIO;

        let randomized_damage = base_damage as f32 + rng.random_range(-variation..variation);

        randomized_damage.round() as u16
    }

    pub(crate) fn get_speed(&self) -> u16 {
        self.base_stats.bind().get_speed()
    }
}