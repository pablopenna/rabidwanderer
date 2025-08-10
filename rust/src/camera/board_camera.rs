use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct BoardCamera {
    #[export]
    focus_move_speed: f32,
    #[export]
    focus_target: Option<Gd<Node2D>>,
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for BoardCamera {
    fn init(base: Base<Camera2D>) -> Self {
        Self {
            focus_move_speed: 1.0,
            focus_target: Option::default(),
            base
        }
    }

    fn process(&mut self, delta: f32) {
        self.focus_target.clone().inspect(|target| {
            let mut cam = self.base_mut().to_godot();
            
            let original_position = cam.get_position();
            let target_position = original_position.lerp(
                target.get_position(), 
                self.focus_move_speed * delta
            );

            cam.set_position(target_position);
        });
    }
}
