use godot::prelude::*;

use crate::board::board::Board;
use crate::board::movement_manager::BoardMovementManager;
use crate::consts::groups::{get_player_node_from_tree, GAME_MANAGER_GROUP};
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
        for _ in 0..10  {
            self.add_floor_item_to_random_tile()
        }
    }

    fn place_player_in_starting_point(&mut self) {
        let starting_coordinate = self
            .get_board()
            .unwrap()
            .bind_mut()
            .get_first_traversable_tile_in_board()
            .unwrap()
            .bind_mut()
            .get_coordinates()
            .clone();
        
        let mut player = self.get_player_ref();
        
        let mut movement_manager_bind = self.movement_manager.bind_mut();
        movement_manager_bind.add_entity_to_board_at_coordinate(
            &mut player, 
            starting_coordinate
        );
    }

    fn get_player_ref(&self) -> Gd<BoardEntity> {
        let node = self.base().to_godot().upcast::<Node>();
        let player = get_player_node_from_tree(node);
        player
    }

    fn add_floor_item_to_random_tile(&mut self) {
        let random_coordinate = self
            .get_board()
            .unwrap()
            .bind_mut()
            .get_random_traversable_tile_in_board()
            .unwrap()
            .bind_mut()
            .get_coordinates()
            .clone();
        
        let mut item = self.floor_item_factory.bind_mut().create_random_floor_item();

        self.movement_manager.bind_mut().add_entity_to_board_at_coordinate(&mut item, random_coordinate);
    }
}

