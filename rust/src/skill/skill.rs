use godot::classes::*;
use godot::prelude::*;

use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_implementation::SkillImplementation;
use crate::skill::get_implementation::get_skill_implementation;

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
    implementation: Option<DynGd<Node, dyn SkillImplementation>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Skill {
    fn init(base: Base<Node>) -> Self {
        Self {
            name: SkillDefinition::Tackle, // default, to be overriden later
            icon: OnEditor::default(),
            implementation: None,
            base,
        }
    }
}

#[godot_api]
impl Skill {
    // https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.DynGd.html
    // Important: caller should add node returned to the tree. Not doing that will cause undesired behaviours in the game
    pub(crate) fn get_implementation(&mut self) -> DynGd<Node, dyn SkillImplementation> {
        if self.implementation.is_none() {
            let skill_name = SkillDefinition::from_gstring(self.get_name());
            let implementation = get_skill_implementation(skill_name);
            self.base_mut().add_child(&implementation);
            self.implementation = Some(implementation);
        }
        self.implementation.clone().unwrap()
    }
}
