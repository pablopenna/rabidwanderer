use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::get_movement_manager_node_from_tree;
use crate::board::movement_manager::BoardMovementManager;
use crate::entity::board_entity::BoardEntity;
use crate::global_signals::GlobalSignals;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerInputModule {
    entity_to_move: Option<Gd<BoardEntity>>,
    is_entity_dead: bool,
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
            is_entity_dead: false,
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
        self.movement_manager = Some(get_movement_manager_node_from_tree(&node));

        GlobalSignals::get_singleton().signals().game_over().connect_other(self, Self::on_entity_death);
        GlobalSignals::get_singleton().signals().battle_finished().connect_other(self, Self::enable);
        // If the deferred flag is not used here, a borrower exception occurs at runtime. Unsure why.
        GlobalSignals::get_singleton().signals().battle_started().builder()
        .flags(ConnectFlags::DEFERRED)
        .connect_other_mut(self, Self::disable);
        
    }

    // https://docs.godotengine.org/en/stable/tutorials/inputs/inputevent.html
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
        
        
        let mut entity = self.entity_to_move.clone().unwrap();
        let direction = Vector2i {
            x: input_dir.x.ceil() as i32,
            y: input_dir.y.ceil() as i32,
        };

        self.movement_manager.clone().unwrap().bind_mut().move_entity_in_board(
            &mut entity,
            direction
        );
    }
}

impl PlayerInputModule {
    fn disable(&mut self) {
        self.base_mut().set_process_unhandled_input(false);
    }

    fn enable(&mut self) {
        if self.is_entity_dead {
            return;    
        }
        self.base_mut().set_process_unhandled_input(true);
    }

    fn on_entity_death(&mut self) {
        self.is_entity_dead = true;
        self.disable();
    }
}
