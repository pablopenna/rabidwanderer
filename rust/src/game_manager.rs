use godot::prelude::*;

use crate::board::board::Board;
use crate::board::movement_manager::BoardMovementManager;
use crate::player::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct GameManager {
    base: Base<Node>,
    #[export]
    board_scene: OnEditor<Gd<PackedScene>>,
    board: Option<Gd<Board>>,
    #[export]
    player_scene: OnEditor<Gd<PackedScene>>,
    player: Option<Gd<Player>>,
    movement_manager: Gd<BoardMovementManager>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            board_scene: OnEditor::default(),
            board: Option::None, // set in ready after instantiating scenes
            player_scene: OnEditor::default(),
            player: Option::None, // set in ready after instantiating scenes
            movement_manager: BoardMovementManager::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        let board = self.board_scene.instantiate_as::<Board>();
        self.base_mut().add_child(&board);
        self.board = Some(board);

        {
            let mut movement_manager_ref = self.movement_manager.bind_mut();
            movement_manager_ref.set_board(self.board.clone());
        }
        
        {
            let movement_manager_node = self.movement_manager.clone().upcast::<Node>();
            self.base_mut().add_child(&movement_manager_node);
        }

        {
            let mut player = self.player_scene.instantiate_as::<Player>();
            player.bind_mut().set_board_movement_manager(Some(self.movement_manager.clone()));
            self.base_mut().add_child(&player);
            self.player = Some(player);
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
        let mut board_bind = self.board.clone().unwrap();
        let mut board = board_bind.bind_mut();
        let traversable_coordinate = 
            board
            .get_first_traversable_tile_coordinates_in_board()
            .unwrap();

        godot_print!("Let's go");

        // let player_gd = self.player.as_mut().unwrap();
        // let mut player_mut = player_gd.bind_mut();
        // movement_manager_bind.add_entity_to_board_at_coordinate(
        //     &mut *player_mut, 
        //     BoardCoordinate::from_vector2i(traversable_coordinate));
        
        let player = &self.player;
        let player_unwrap = &mut player.clone().unwrap();
        let mut player_bind = player_unwrap.bind_mut();
        let player_entity = &mut *player_bind;
        
        let mut movement_manager_bind = self.movement_manager.bind_mut();
        movement_manager_bind.add_entity_to_board_at_coordinate(
            player_entity, 
            traversable_coordinate
        );
    }
}

