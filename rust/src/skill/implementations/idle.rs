use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct IdleSkillImplementation {
    base: Base<Node>,
}

#[godot_dyn]
impl SkillImplementation for IdleSkillImplementation {
    fn cast(&mut self, user: Gd<BattleEntity>, targets: &Array<Gd<BattleEntity>>) {
        godot_print!("idleada de manual. nº targets: {}", targets.len());
        godot_print!("I'm catching my breath");

        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_callback(&Callable::from_object_method(
            &user,
            "on_skill_casting_done",
        ));
    }
}
