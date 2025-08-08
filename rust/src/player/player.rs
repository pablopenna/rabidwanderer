use godot::classes::*;
use godot::prelude::*;

use crate::board::coordinate::BoardCoordinate;
use crate::board::entity::BoardEntity;
use crate::board::movement_manager::BoardMovementManager;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Player {
    coordinates: BoardCoordinate,
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

        self.get_board_movement_manager()
            .unwrap()
            .bind_mut()
            .move_entity_in_board(self, Vector2i {
                x: input_dir.x.ceil() as i32,
                y: input_dir.y.ceil() as i32,
            });
    }
}

impl BoardEntity for Player {
    fn get_coordinates(&self) -> &BoardCoordinate {
        &self.coordinates
    }
    
    fn set_coordinates(&mut self, coord: BoardCoordinate) {
        self.coordinates = coord
    }

    fn set_world_position(&mut self, position: Vector2) {
        let global_position = self.base().to_godot().to_global(position);
        self.base_mut().to_godot().set_position(global_position);
    }
}
