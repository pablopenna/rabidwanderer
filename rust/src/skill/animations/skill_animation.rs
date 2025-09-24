use godot::classes::*;
use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, Default)]
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
