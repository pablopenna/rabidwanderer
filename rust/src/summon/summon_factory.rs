use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;
use crate::consts::groups::SUMMON_FACTORY_GROUP;
use crate::summon::summon_definition::SummonDefinition;
use crate::summon::summoned_entity::SummonedEntity;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct SummonFactory {
    #[export]
    summoning_spirit_scene: OnEditor<Gd<PackedScene>>,
    summoning_spirit_instance: Option<Gd<SummonedEntity>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SummonFactory {
    fn ready(&mut self) {
        self.base_mut().add_to_group(SUMMON_FACTORY_GROUP);
    }
}

#[godot_api]
impl SummonFactory {
    pub(crate) fn generate_summoned_battle_entity(
        &mut self,
        id: SummonDefinition,
        team: Team,
    ) -> Gd<BattleEntity> {
        match id {
            SummonDefinition::Spirit => self
                .get_summoning_spirit_instance()
                .bind_mut()
                .generate_battle_entity(team),
        }
    }

    fn get_summoning_spirit_instance(&mut self) -> Gd<SummonedEntity> {
        if self.summoning_spirit_instance.is_none() {
            let instance = self
                .get_summoning_spirit_scene()
                .unwrap()
                .instantiate_as::<SummonedEntity>();
            self.base_mut().add_child(&instance); // It is important to add to tree because stats and skills nodes are not children of the BattleEntity although used by it. If not, weird behaviour will happen like Tweens not working.
            self.summoning_spirit_instance = Some(instance);
        }
        self.summoning_spirit_instance.clone().unwrap()
    }
}
