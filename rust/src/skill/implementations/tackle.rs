use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct TackleSkillImplementation {
    base: Base<Node>
}

#[godot_dyn]
impl SkillImplementation for TackleSkillImplementation {
    // https://github.com/godot-rust/gdext/issues/1318
    // https://godot-rust.github.io/docs/gdext/master/godot/prelude/attr.godot_api.html#associated-functions-and-methods
    // Calling tween_property on the user.sprite triggers an already_bound error. Using #[func(gd_self)] fixes the issue.
    fn cast(&mut self, user: Gd<BattleEntity>, target: Gd<BattleEntity>) {
        godot_print!("tackleada de manual");
        let mut tween = self.base_mut().create_tween().unwrap();
        // let skill_callable = Callable::from_object_method(&self.to_gd(), "cast_skill");
        // let skill_callable_with_args = skill_callable.bind(&[skill.to_variant()]);
        tween.tween_property(
            &user.bind().get_sprite().unwrap(),
            "scale", 
            &Vector2{x: 2.0, y: 2.0}.to_variant(), 
            1.0
        );
        tween.tween_property(
            &user.bind().get_sprite().unwrap(),
            "scale", 
            &Vector2{x: 1.0, y: 1.0}.to_variant(), 
            0.5
        );
        tween.tween_callback(&Callable::from_object_method(&user, "on_apply_damage"));
        tween.tween_callback(&Callable::from_object_method(&user, "on_skill_casting_done"));//.unwrap().set_delay(1.0);
    }
}