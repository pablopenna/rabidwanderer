use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct BiteSkill {
    base: Base<Node>
}

#[godot_dyn]
impl SkillImplementation for BiteSkill {
    fn cast(&mut self, user: Gd<BattleEntity>, target: Gd<BattleEntity>) {
        
    }
}