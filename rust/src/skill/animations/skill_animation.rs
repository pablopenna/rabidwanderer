use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;

#[derive(GodotConvert, Var, Export, Clone, Default, PartialEq)]
#[godot(via = GString)] 
enum AnimationFacingDirection {
    Left,
    #[default]
    Right
}

#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
pub(crate) struct SkillAnimation {
    #[export]
    facing_direction: AnimationFacingDirection,
    base: Base<Sprite2D>,
}

impl SkillAnimation {
    pub(crate) fn adapt_facing_direction_to_target(&mut self, user: &Gd<BattleEntity>, target: &Gd<BattleEntity>) {
        if self.facing_direction == AnimationFacingDirection::Left && SkillAnimation::is_user_to_the_left_of_target(user, target) {
            self.base_mut().set_flip_h(true);
        }

        if self.facing_direction == AnimationFacingDirection::Right && !SkillAnimation::is_user_to_the_left_of_target(user, target) {
            self.base_mut().set_flip_h(true);
        }
    }

    fn is_user_to_the_left_of_target(user: &Gd<BattleEntity>, target: &Gd<BattleEntity>) -> bool {
        user.get_global_position().x < target.get_global_position().x
    }
}