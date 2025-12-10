use godot::prelude::*;

use crate::stats::stat::Stat;
use crate::battle::entity::entity::BattleEntity;

pub(crate) fn apply_damage(actor: Gd<BattleEntity>, targets: Array<Gd<BattleEntity>>) {
    godot_print!("[{}] I'm attacking", actor.get_name());
    let attack_damage = actor.bind().get_stats().bind().get_stat(Stat::Attack);
    godot_print!(
        "[{}] I am about to deal {} damage",
        actor.get_name(),
        attack_damage
    );

    targets.iter_shared().for_each(|mut target| {
        godot_print!(
            "[{}] I am about to attack {}",
            actor.get_name(),
            target.get_name()
        );

        target.bind_mut().take_damage(attack_damage.max(0) as u16);
    });
}

/** 
 * Wrapper around apply_damage() to be used as Callable 
 * */
pub(crate) fn apply_damage_variant(args: &[&Variant]) {
    let actor = args[0].to::<Gd<BattleEntity>>();
    let targets = args[1].to::<Array<Gd<BattleEntity>>>();
    
    apply_damage(actor, targets);
}