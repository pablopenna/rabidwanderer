use std::{cell::RefCell, rc::Rc};

use godot::{classes::Node, obj::Gd};

use crate::{board::movement_manager::BoardMovementManager, entity::board_entity::BoardEntity, game_manager::{self, GameManager}};

pub(crate) const MOVEMENT_MANAGER_GROUP: &str = "MovementManager";
// IMPORTANT: make sure the Player scene has the Player group assigned
pub(crate) const PLAYER_GROUP: &str = "Player";
pub(crate) const GAME_MANAGER_GROUP: &str = "GameManager";

pub(crate) fn get_node_in_group_from_tree(node: Gd<Node>, group_name: &str) -> Gd<Node> {
    let movement_node = node.get_tree().unwrap().get_first_node_in_group(group_name).unwrap();
    movement_node.cast::<Node>()
}

pub(crate) fn get_movement_manager_node_from_tree(node: Gd<Node>) -> Gd<BoardMovementManager> {
    let move_manager = get_node_in_group_from_tree(node, MOVEMENT_MANAGER_GROUP);
    move_manager.cast::<BoardMovementManager>()
}

pub(crate) fn get_game_manager_node_from_tree(node: Gd<Node>) -> Gd<GameManager> {
    let game_manager = get_node_in_group_from_tree(node, GAME_MANAGER_GROUP);
    game_manager.cast::<GameManager>()
}

pub(crate) fn get_player_node_from_tree(node: Gd<Node>) -> Gd<BoardEntity> {
    let player = get_node_in_group_from_tree(node, PLAYER_GROUP);
    player.cast::<BoardEntity>()
}

pub(crate) fn get_player_ref_from_tree(node: Gd<Node>) -> Rc<RefCell<Gd<BoardEntity>>> {
    let player = get_node_in_group_from_tree(node, PLAYER_GROUP);
    let player = player.cast::<BoardEntity>();
    Rc::new(RefCell::new(player))
}