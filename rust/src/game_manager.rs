use std::cell::RefCell;
use std::rc::Rc;

use godot::prelude::*;

use crate::board::board::Board;
use crate::board::coordinate::BoardCoordinate;
use crate::board::movement_manager::BoardMovementManager;
use crate::consts::groups::PLAYER_GROUP;
use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct GameManager {
    base: Base<Node>,
    #[export]
    // https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.OnEditor.html#custom-getters-and-setters-for-oneditor
    board: OnEditor<Gd<Board>>,
    #[export]
    player_scene: OnEditor<Gd<PackedScene>>,
    player: Option<Rc<RefCell<Gd<BoardEntity>>>>,
    movement_manager: Gd<BoardMovementManager>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            board: OnEditor::default(),
            player_scene: OnEditor::default(),
            player: Option::None, // set in ready after instantiating scenes
            movement_manager: BoardMovementManager::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        {
            let mut movement_manager_ref = self.movement_manager.bind_mut();
            let board = self.board.get_property().unwrap();
            movement_manager_ref.set_board(Option::Some(board));
        }
        
        {
            let movement_manager_node = self.movement_manager.clone().upcast::<Node>();
            self.base_mut().add_child(&movement_manager_node);
        }

        {
            let player_instance = self.player_scene.instantiate_as::<BoardEntity>();
            let player_ref = Rc::new(RefCell::new(player_instance));
            self.player = Some(player_ref.clone());
            
            let player_node = player_ref.clone();
            let player_node = player_node.borrow_mut();
            let mut player_node = player_node.clone().upcast::<Node>();
            player_node.add_to_group(PLAYER_GROUP);
            self.base_mut().add_child(&player_node);
        }

        self.signals().game_ready().connect_self(Self::on_game_ready);
        self.signals().game_ready().emit();
    }
}

#[godot_api]
impl GameManager {
    #[signal]
    pub fn game_ready();

    fn on_game_ready(&mut self) {
        let board = self.board.get_property();
        let board = board.unwrap();
        let board = board.bind();
        let traversable_coordinate = 
            board
            .get_first_traversable_tile_coordinates_in_board()
            .unwrap();

        // HACK: Compiler does not complain but game crashes on startup without the drop() below
        /* ERROR: godot-rust function call failed: <Callable>::my_rust_lib::game_manager::GameManager::on_game_ready()
        Reason: function panicked: Gd<T>::bind_mut() failed, already bound; T = my_rust_lib::board::board::Board.
        Make sure to use `self.base_mut()` instead of `self.to_gd()` when possible.
        Details: cannot borrow mutable while shared borrow exists. */ 
        drop(board);

        godot_print!("Let's go");
        
        let player = self.get_player_ref();
        
        let mut movement_manager_bind = self.movement_manager.bind_mut();
        movement_manager_bind.add_entity_to_board_at_coordinate(
            player, 
            traversable_coordinate
            // BoardCoordinate::from_index(0)
        );
    }

    fn get_player_ref(&self) -> Rc<RefCell<Gd<BoardEntity>>> {
        self.player.clone().unwrap()
    }
}

