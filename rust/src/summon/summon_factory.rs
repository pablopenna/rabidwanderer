use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::consts::groups::SUMMON_FACTORY_GROUP;
use crate::summon::summon_definition::SummonDefinition;
use crate::summon::summoned_entity::SummonedEntity;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct SummonFactory {
    #[export]
    spirit_scene: OnEditor<Gd<PackedScene>>,
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
    pub(crate) fn generate_summoned_battle_entity(&self, id: SummonDefinition) -> Gd<BattleEntity> {
        match id {
            SummonDefinition::Spirit => self.instance_spirit_summoning_scene().bind_mut().generate_battle_entity()
        }
    }
    
    fn instance_spirit_summoning_scene(&self) -> Gd<SummonedEntity> {
        self.get_spirit_scene().unwrap().instantiate_as::<SummonedEntity>()
    }
}