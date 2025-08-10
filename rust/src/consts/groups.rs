use godot::{classes::Node, obj::Gd};

use crate::board::movement_manager::BoardMovementManager;

pub(crate) const MOVEMENT_MANAGER_GROUP: &str = "MovementManager";
pub(crate) const PLAYER_GROUP: &str = "Player";

pub(crate) fn get_node_in_group_from_tree(node: Gd<Node>, group_name: &str) -> Gd<Node> {
    let movement_node = node.get_tree().unwrap().get_first_node_in_group(group_name).unwrap();
    movement_node.cast::<Node>()
}

pub(crate) fn get_movement_manager_node_from_tree(node: Gd<Node>) -> Gd<BoardMovementManager> {
    let move_manager = get_node_in_group_from_tree(node, MOVEMENT_MANAGER_GROUP);
    move_manager.cast::<BoardMovementManager>()
}