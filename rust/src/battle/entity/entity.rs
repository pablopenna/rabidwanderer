use godot::classes::*;
use godot::prelude::*;

use rand::Rng;

use crate::stats::real::RealStats;
use crate::battle::team::Team;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub(crate) struct BattleEntity {
    base: Base<Node2D>,
    #[export]
    stats: OnEditor<Gd<RealStats>>,
    #[export]
    team: Team,
    #[export]
    target: Option<Gd<BattleEntity>>,
    #[export]
    animation_player: OnEditor<Gd<AnimationPlayer>>
}

#[godot_api]
impl BattleEntity {
    #[signal]
    pub(crate) fn death();

    #[signal]
    pub(crate) fn done_acting();

    pub(crate) fn take_damage(&mut self, damage: u16) {
        let hp = self.stats.bind_mut().get_current_hp();
        if hp > damage {
            self.stats.bind_mut().set_current_hp(hp - damage);
        } else {
            self.stats.bind_mut().set_current_hp(0);
            self.signals().death().emit();
        }
        godot_print!("Ouch! I have {} remaining", self.stats.bind().get_current_hp());
    }

    pub(crate) fn act(&mut self) {
        // Once animations are added, these methods should be called from the animation.
        // This method then would only kickstart the animation
        // self.on_apply_damage();
        // self.on_done_acting();

        self.get_animation_player().unwrap().play_ex().name("attack").done();
    }

    #[func]
    fn on_apply_damage(&mut self) {
        godot_print!("I am {} and I'm attacking", self.base().get_name());
        let attack_damage = self.calculate_attack_damage();
        self.target.clone().unwrap().bind_mut().take_damage(attack_damage);
    }

    #[func]
    fn on_done_acting(&mut self) {
        godot_print!("I am {} and I'm done", self.base().get_name());
        self.signals().done_acting().emit();
    }

    fn calculate_attack_damage(&self) -> u16 {
        let base_damage = self.stats.bind().get_attack();
        let mut rng = rand::rng();
        let variation_ratio: f32 = base_damage as f32 * 0.1;

        let randomized_damage = base_damage as f32 + rng.random_range(-variation_ratio..variation_ratio);

        randomized_damage.round() as u16
    }
}


