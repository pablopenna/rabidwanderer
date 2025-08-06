use godot::classes::*;
use godot::prelude::*;

use crate::board::board_movement_manager::BoardMovementManager;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Player {
    // https://github.com/godot-rust/gdext/issues/972
    // https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#exporting
    #[export]
    sprite: OnEditor<Gd<Sprite2D>>,
    #[export]
    board_movement_manager: OnEditor<Gd<BoardMovementManager>>,
    base: Base<Node2D>,
}

const INPUT_LEFT: &'static str = "ui_left";
const INPUT_RIGHT: &'static str = "ui_right";
const INPUT_UP: &'static str = "ui_up";
const INPUT_DOWN: &'static str = "ui_down";

#[godot_api]
impl INode2D for Player {
    //fn physics_process(&mut self, delta: f32) {
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if !event.is_action(INPUT_LEFT)
            && !event.is_action(INPUT_RIGHT)
            && !event.is_action(INPUT_UP)
            && !event.is_action(INPUT_DOWN)
        {
            return;
        }

        let input_dir =
            Input::singleton().get_vector(INPUT_LEFT, INPUT_RIGHT, INPUT_UP, INPUT_DOWN);
        
        if input_dir.is_zero_approx() {
            return;
        }

        self.board_movement_manager
            .signals()
            .entity_move_intent()
            .emit(&self.to_gd(), input_dir.normalized());
    }
}
