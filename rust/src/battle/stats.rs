use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub(crate) struct BattleStats {
    base: Base<Resource>,
    #[export]
    max_hp: u16,
    #[export]
    current_hp: u16,
    #[export]
    attack: u16,
    #[export]
    defense: u16,
    #[export]
    speed: u16,
}

#[godot_api]
impl IResource for BattleStats {

}

#[godot_api]
impl BattleStats {
    pub(crate) fn _new(max_hp: u16, attack: u16, defense: u16, speed: u16) -> Gd<BattleStats> {
        let mut new_stats = Self::new_gd();
        new_stats.bind_mut().set_max_hp(max_hp);
        new_stats.bind_mut().set_current_hp(max_hp);
        new_stats.bind_mut().set_attack(attack);
        new_stats.bind_mut().set_defense(defense);
        new_stats.bind_mut().set_speed(speed);
        new_stats
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}