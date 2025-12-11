use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::implementations::utils::apply_damage::apply_damage_variant;
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct SonicPunchSkillImplementation {
    base: Base<Node>,
}

#[godot_dyn]
impl SkillImplementation for SonicPunchSkillImplementation {
    // https://github.com/godot-rust/gdext/issues/1318
    // https://godot-rust.github.io/docs/gdext/master/godot/prelude/attr.godot_api.html#associated-functions-and-methods
    // Calling tween_property on the user.sprite triggers an already_bound error. Using #[func(gd_self)] fixes the issue.
    fn cast(&mut self, user: Gd<BattleEntity>, targets: &Array<Gd<BattleEntity>>) {
        godot_print!("puncheada de manual. nº targets: {}", targets.len());

        let user_position = user.get_global_position();
        let target = targets.get(0).unwrap();
        let target_position = target.get_global_position();

        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_property(
            &user.bind().get_sprite().unwrap(),
            "global_position",
            &target_position.to_variant(),
            0.1,
        );
        tween.tween_property(
            &user.bind().get_sprite().unwrap(),
            "global_position",
            &user_position.to_variant(),
            0.1,
        );
        tween.tween_callback(
            &Callable::from_fn("apply_damage", apply_damage_variant)
                .bind(&[user.to_variant(), targets.to_variant()]),
        );
        tween.tween_callback(&Callable::from_object_method(
            &user,
            "on_skill_casting_done",
        )); //.unwrap().set_delay(1.0);
    }
}
