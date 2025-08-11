use std::cell::RefCell;
use std::rc::Rc;

use godot::prelude::*;

use crate::board::board::Board;
use crate::board::movement_manager::BoardMovementManager;
use crate::consts::groups::{get_player_ref_from_tree, GAME_MANAGER_GROUP};
use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::item::floor_item_factory::FloorItemFactory;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct GameManager {
    base: Base<Node>,
    #[export]
    // https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.OnEditor.html#custom-getters-and-setters-for-oneditor
    board: OnEditor<Gd<Board>>,
    movement_manager: Gd<BoardMovementManager>,
    #[export]
    floor_item_factory: OnEditor<Gd<FloorItemFactory>>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            board: OnEditor::default(),
            movement_manager: BoardMovementManager::new_alloc(),
            base,
            floor_item_factory: OnEditor::default(),
        }
    }

    fn ready(&mut self) {
        {
            let mut node = self.base().to_godot().upcast::<Node>();
            node.add_to_group(GAME_MANAGER_GROUP);
        }
        
        {
            let mut movement_manager_ref = self.movement_manager.bind_mut();
            let board = self.board.get_property().unwrap();
            movement_manager_ref.set_board(Option::Some(board));
        }
        
        {
            let movement_manager_node = self.movement_manager.clone().upcast::<Node>();
            self.base_mut().add_child(&movement_manager_node);
        }

        self.on_game_ready();
        self.signals().game_ready().emit();
    }
}

#[godot_api]
impl GameManager {
    #[signal]
    pub fn game_ready();

    fn on_game_ready(&mut self) {
        self.place_player_in_starting_point();
        for _ in 0..5  {
            self.add_floor_item_to_random_tile()
        }
    }

    fn place_player_in_starting_point(&mut self) {
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
        let node = self.base().to_godot().upcast::<Node>();
        let player = get_player_ref_from_tree(node);
        player
    }

    fn add_floor_item_to_random_tile(&mut self) {
        let board = self.board.get_property();
        let mut board = board.unwrap();
        let mut board = board.bind_mut();
        let traversable_coordinate = 
            board
            .get_random_traversable_tile_coordinates_in_board()
            .unwrap();
        // HACK: Compiler does not complain but game crashes on startup without the drop() below
        drop(board);
        
        let mut node = self.base_mut().to_godot().upcast::<Node>();
        let item = self.floor_item_factory.bind_mut().create_random_floor_item();
        node.add_child(&item);

        let item = Rc::new(RefCell::new(item));
        self.movement_manager.bind_mut().add_entity_to_board_at_coordinate(item, traversable_coordinate);
    }
}

