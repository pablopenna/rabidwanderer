use std::cell::RefCell;
use std::rc::Rc;

use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::get_movement_manager_node_from_tree;
use crate::board::movement_manager::BoardMovementManager;
use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerInputModule {
    entity_to_move: Option<Gd<BoardEntity>>,
    movement_manager: Option<Gd<BoardMovementManager>>,
    base: Base<Node2D>,
}

const INPUT_LEFT: &'static str = "ui_left";
const INPUT_RIGHT: &'static str = "ui_right";
const INPUT_UP: &'static str = "ui_up";
const INPUT_DOWN: &'static str = "ui_down";

#[godot_api]
impl INode2D for PlayerInputModule {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            entity_to_move: Option::None,
            movement_manager: Option::None,
            base
        }
    }

    fn ready(&mut self) {
        // IMPORTANT: Assume parent is the BoardEntity that this module is linked to
        let parent = self.base().get_parent();
        let entity = parent.unwrap().cast::<BoardEntity>();
        self.entity_to_move = Some(entity);

        let node = self.base().clone().upcast::<Node>();
        self.movement_manager = Some(get_movement_manager_node_from_tree(node));
    }

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
        
        
        let entity = self.entity_to_move.clone().unwrap();
        let entity_reference = Rc::new(RefCell::new(entity));
        let direction = Vector2i {
            x: input_dir.x.ceil() as i32,
            y: input_dir.y.ceil() as i32,
        };

        self.movement_manager.clone().unwrap().bind_mut().move_entity_in_board(
            entity_reference,
            direction
        );
    }
}
