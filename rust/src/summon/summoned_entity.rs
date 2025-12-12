use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::battle::battle::BattleModule;

/** A summoned entity is like a BoardEntity in the sense that it spawns a BattleEntity but
 * with the difference that it does not appear in the board.
 *
 * Summoned entities are invoked during battle to help its summoner.
 *
 * This class is mostly just a wrapper for a BattleModule.
 */
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SummonedEntity {
    #[export]
    battle_module: OnEditor<Gd<BattleModule>>,
    base: Base<Node>,
}

#[godot_api]
impl SummonedEntity {
    pub(crate) fn generate_battle_entity(&mut self) -> Gd<BattleEntity> {
        self.battle_module.bind_mut().generate_new_battle_entity_instance()
    }
}
