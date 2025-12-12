use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::consts::groups::get_battle_setup_node_from_tree;
use crate::consts::groups::get_summon_factory_node_from_tree;
use crate::skill::skill_implementation::SkillImplementation;
use crate::summon::summon_definition::SummonDefinition;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct SummonSpiritSkillImplementation {
    base: Base<Node>,
}

#[godot_dyn]
impl SkillImplementation for SummonSpiritSkillImplementation {
    fn cast(&mut self, user: Gd<BattleEntity>, targets: &Array<Gd<BattleEntity>>) {
        godot_print!("sumoneada de manual. nº targets: {}", targets.len());

        let team = user.bind().get_entity_team();

        let mut summon_factory = get_summon_factory_node_from_tree(&self.base());
        let spirit = summon_factory
            .bind_mut()
            .generate_summoned_battle_entity(SummonDefinition::Spirit, team);

        let mut battle_setup = get_battle_setup_node_from_tree(&self.base());
        battle_setup.bind_mut().add_entity_to_combat(spirit);

        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_callback(&Callable::from_object_method(
            &user,
            "on_skill_casting_done",
        ));
    }
}
