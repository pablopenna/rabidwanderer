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
    name: SkillDefinition,
    #[export]
    icon: OnEditor<Gd<Texture2D>>,
    #[export]
    target_amount: TargetAmount,
    #[export]
    target_faction: TargetFaction,
    implementation: Option<DynGd<Node, dyn SkillImplementation>>,
    #[export]
    priority: i32,
    base: Base<Node>,
}

#[godot_api]
impl INode for Skill {
    fn init(base: Base<Node>) -> Self {
        Self {
            name: SkillDefinition::Tackle, // default, to be overriden later
            icon: OnEditor::default(),
            target_amount: TargetAmount::Single, // default, to be overriden later
            target_faction: TargetFaction::Opponent, // default, to be overriden later
            implementation: None,
            priority: 0,
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
