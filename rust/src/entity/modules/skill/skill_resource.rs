use godot::classes::*;
use godot::prelude::*;

use crate::skill::resource::manager::SkillResourceManager;
use crate::skill::skill_definition::SkillDefinition;

/*
* This is acts as an interface for managing the resources
* required for casting skills. The actual implementation and logic will be a child of this node.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct SkillResourceModule {
    #[export]
    implementation: Option<DynGd<Node, dyn SkillResourceManager>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillResourceModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            implementation: None,
        }
    }
}

#[godot_api]
impl SkillResourceModule {
    pub(crate) fn consume_resources_for_casting(&self, skill: SkillDefinition) {
        let mut implementation = self.implementation.clone().unwrap();
        implementation.dyn_bind_mut().consume_resources_for_casting(skill);
    }

    pub(crate) fn has_resources_to_cast(&self, skill: SkillDefinition) -> bool {
        let mut implementation = self.implementation.clone().unwrap();
        let can_cast = implementation.dyn_bind_mut().has_resources_to_cast(skill);

        can_cast
    }
}
