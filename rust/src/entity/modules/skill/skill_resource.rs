use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::resource::manager::SkillResourceManager;
use crate::skill::skill_definition::SkillDefinition;

/*
* This is acts as an interface for managing the resources
* required for casting skills. The actual implementation and logic will be a child of this node.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct SkillResourceModule {
    // HACK: if I call it implementation, get_implementation() is automatically generated
    // due to #[export] and it breaks the DynGd part: the return type of that generated
    // function becomes Option<Gd<Node>>.
    #[export]
    _implementation: Option<DynGd<Node, dyn SkillResourceManager>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillResourceModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            _implementation: Option::default(),
        }
    }
}

#[godot_api]
impl SkillResourceModule {
    #[signal]
    pub(crate) fn added_to_battle_entity(entity: Gd<BattleEntity>);

    pub(crate) fn consume_resources_for_casting(&mut self, skill: SkillDefinition) {
        self._implementation
            .clone()
            .unwrap()
            .dyn_bind_mut()
            .consume_resources_for_casting(skill);
    }

    pub(crate) fn has_resources_to_cast(&self, skill: SkillDefinition) -> bool {
        self._implementation
            .clone()
            .unwrap()
            .dyn_bind()
            .has_resources_to_cast(skill)
    }

    pub(crate) fn get_implementation(&self) -> DynGd<Node, dyn SkillResourceManager> {
        self._implementation.clone().unwrap()
    }
}
