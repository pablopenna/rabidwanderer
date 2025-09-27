use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::consts::groups::get_skill_animation_factory_node_from_tree;
use crate::skill::animations::skill_animation_factory::SkillAnimationName;
use crate::skill::skill_implementation::SkillImplementation;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct BiteSkillImplementation {
    base: Base<Node>
}

#[godot_dyn]
impl SkillImplementation for BiteSkillImplementation {
    fn cast(&mut self, mut user: Gd<BattleEntity>, target: Gd<BattleEntity>) {
        godot_print!("biteada de manual");
        let node = self.base_mut().clone().upcast::<Node>();
        let animation_factory = get_skill_animation_factory_node_from_tree(node);
        let mut animation = animation_factory.bind().get_animation(SkillAnimationName::Bite);
        user.add_child(&animation);

        animation.bind_mut().adapt_facing_direction_to_target(&user, &target);
        animation.set_global_position(target.get_global_position());
        
        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_property(
            &animation,
            "frame", 
            &Variant::from(4), 
            1.0
        );
        tween.tween_callback(&Callable::from_object_method(&animation, "queue_free"));
        tween.tween_callback(&Callable::from_object_method(&user, "on_apply_damage"));
        tween.tween_callback(&Callable::from_object_method(&user, "on_done_acting")).unwrap().set_delay(1.0);
    }
}