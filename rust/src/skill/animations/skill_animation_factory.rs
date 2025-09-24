use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::SKILL_ANIMATION_FACTORY_GROUP;
use crate::skill::animations::skill_animation::SkillAnimation;

pub(crate) enum SkillAnimationName {
    Bite,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct SkillAnimationFactory {
    #[export]
    bite_animation_scene: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillAnimationFactory {
    fn ready(&mut self) {
        let mut node = self.base_mut().to_godot().upcast::<Node>();
        node.add_to_group(SKILL_ANIMATION_FACTORY_GROUP);
    }
}

impl SkillAnimationFactory {
    pub(crate) fn get_animation(&self, animation_name: SkillAnimationName) -> Gd<SkillAnimation> {
        let scene = match animation_name {
            SkillAnimationName::Bite => self.get_bite_animation_scene(),
        };

        let animation = scene.unwrap().instantiate_as::<SkillAnimation>();
        animation
    }
}
