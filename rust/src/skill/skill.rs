use godot::classes::*;
use godot::prelude::*;

use crate::skill::get_implementation::get_skill_implementation;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_implementation::SkillImplementation;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_faction::TargetFaction;

/*
* This is a template for a skill.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Skill {
    #[export]
    _definition: SkillDefinition,
    #[export]
    icon: OnEditor<Gd<Texture2D>>,
    implementation: Option<DynGd<Node, dyn SkillImplementation>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Skill {
    fn init(base: Base<Node>) -> Self {
        Self {
            _definition: SkillDefinition::Tackle, // default, to be overriden later
            icon: OnEditor::default(),
            implementation: None,
            base,
        }
    }
}

#[godot_api]
impl Skill {
    pub(crate) fn get_implementation(&mut self) -> DynGd<Node, dyn SkillImplementation> {
        if self.implementation.is_none() {
            let skill_name = self.get_definition();
            let implementation = get_skill_implementation(skill_name);
            self.base_mut().add_child(&implementation);
            self.implementation = Some(implementation);
        }
        self.implementation.clone().unwrap()
    }

    pub(crate) fn get_definition(&self) -> SkillDefinition {
        self._definition.clone()
    }

    pub(crate) fn get_description(&self) -> &'static str {
        self._definition.get_description()
    }

    pub(crate) fn get_cooldown(&self) -> u8 {
        self._definition.get_cooldown()
    }

    pub(crate) fn get_target_amount(&self) -> TargetAmount {
        self._definition.get_target_amount()
    }

    pub(crate) fn get_target_faction(&self) -> TargetFaction {
        self._definition.get_target_faction()
    }

    pub(crate) fn get_priority(&self) -> i32 {
        self._definition.get_priority()
    }
}
