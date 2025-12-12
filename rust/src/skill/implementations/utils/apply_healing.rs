use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::stats::stat::Stat;

pub(crate) fn apply_healing(actor: Gd<BattleEntity>, targets: Array<Gd<BattleEntity>>) {
    godot_print!("[{}] I'm healing", actor.get_name());
    let attack_damage = actor.bind().get_stats().bind().get_stat(Stat::Attack);
    godot_print!(
        "[{}] I am about to heal {} damage",
        actor.get_name(),
        attack_damage
    );

    targets.iter_shared().for_each(|mut target| {
        godot_print!(
            "[{}] I am about to heal {}",
            actor.get_name(),
            target.get_name()
        );

        target.bind_mut().heal_damage(attack_damage.max(0) as u16);
    });
}

/**
 * Wrapper around apply_damage() to be used as Callable
 * */
pub(crate) fn apply_healing_variant(args: &[&Variant]) {
    let actor = args[0].to::<Gd<BattleEntity>>();
    let targets = args[1].to::<Array<Gd<BattleEntity>>>();

    apply_healing(actor, targets);
}
