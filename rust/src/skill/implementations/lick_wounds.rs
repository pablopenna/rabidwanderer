use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::implementations::utils::apply_healing::apply_healing_variant;
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct LickWoundsSkillImplementation {
    base: Base<Node>,
}

#[godot_dyn]
impl SkillImplementation for LickWoundsSkillImplementation {
    fn cast(&mut self, user: Gd<BattleEntity>, targets: &Array<Gd<BattleEntity>>) {
        godot_print!("lickeada de manual. nº targets: {}", targets.len());

        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_callback(
            &Callable::from_fn("apply_damage", apply_healing_variant)
                .bind(&[user.to_variant(), targets.to_variant()]),
        );
        tween.tween_callback(&Callable::from_object_method(
            &user,
            "on_skill_casting_done",
        ));
    }
}
