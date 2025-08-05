use godot::prelude::*;
use godot::classes::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Player {
    #[export]
    sprite: Option<Gd<Sprite2D>>, // https://github.com/godot-rust/gdext/issues/972
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Player {
    fn physics_process(&mut self, delta: f32) {
        let input_dir = Input::singleton().get_vector("ui_left", "ui_right", "ui_up", "ui_down");
        let position = self.base().get_position();
        let move_speed: f32 = 100.0;

        self.base_mut().set_position(position + input_dir * move_speed * delta);
    }
}