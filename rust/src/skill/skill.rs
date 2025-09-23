use godot::classes::*;
use godot::prelude::*;

use crate::skill::skill_definition::SkillDefinition;

/* 
* This is a template for a skill.
*/
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Skill {
    #[export]
    name: SkillDefinition,
    #[export]
    icon: OnEditor<Gd<Texture2D>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Skill {
    fn init(base: Base<Node>) -> Self {
        Self {
            name: SkillDefinition::Tackle, // default, to be overriden later
            icon: OnEditor::default(),
            base
        }
    }
}
