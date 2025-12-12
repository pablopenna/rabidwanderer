use godot::classes::Node;
use godot::obj::Gd;

use crate::battle::engine::BattleEngine;
use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::setup::BattleSetup;
use crate::board::board::Board;
use crate::board::movement_manager::BoardMovementManager;
use crate::enemy::factory::EnemyFactory;
use crate::entity::board_entity::BoardEntity;
use crate::game_manager::GameManager;
use crate::item::factory::ItemFactory;
use crate::skill::animations::skill_animation_factory::SkillAnimationFactory;
use crate::skill::skill_factory::SkillFactory;
use crate::summon::summon_factory::SummonFactory;

pub(crate) const MOVEMENT_MANAGER_GROUP: &str = "MovementManager";
// IMPORTANT: make sure the Player scene has the Player group assigned
pub(crate) const PLAYER_GROUP: &str = "Player";
pub(crate) const GAME_MANAGER_GROUP: &str = "GameManager";
pub(crate) const ITEM_FACTORY_GROUP: &str = "ItemFactory";
pub(crate) const SKILL_FACTORY_GROUP: &str = "SkillFactory";
pub(crate) const SKILL_ANIMATION_FACTORY_GROUP: &str = "SkillAnimationFactory";
pub(crate) const ENEMY_FACTORY_GROUP: &str = "EnemyFactory";
pub(crate) const BATTLE_ENGINE_GROUP: &str = "BattleEngine";
pub(crate) const BATTLE_SETUP_GROUP: &str = "BattleSetup";
pub(crate) const BOARD_GROUP: &str = "Board";
pub(crate) const BATTLE_ENTITY_CONTAINER_GROUP: &str = "BattleEntityContainer";
pub(crate) const SUMMON_FACTORY_GROUP: &str = "SummonFactory";

pub(crate) fn get_node_in_group_from_tree(node: &Gd<Node>, group_name: &str) -> Gd<Node> {
    let movement_node = node
        .get_tree()
        .unwrap()
        .get_first_node_in_group(group_name)
        .unwrap();
    movement_node.cast::<Node>()
}

pub(crate) fn get_movement_manager_node_from_tree(node: &Gd<Node>) -> Gd<BoardMovementManager> {
    let move_manager = get_node_in_group_from_tree(node, MOVEMENT_MANAGER_GROUP);
    move_manager.cast::<BoardMovementManager>()
}

pub(crate) fn _get_game_manager_node_from_tree(node: &Gd<Node>) -> Gd<GameManager> {
    let game_manager = get_node_in_group_from_tree(node, GAME_MANAGER_GROUP);
    game_manager.cast::<GameManager>()
}

pub(crate) fn get_player_node_from_tree(node: &Gd<Node>) -> Gd<BoardEntity> {
    let player = get_node_in_group_from_tree(node, PLAYER_GROUP);
    player.cast::<BoardEntity>()
}

pub(crate) fn get_item_factory_node_from_tree(node: &Gd<Node>) -> Gd<ItemFactory> {
    let factory = get_node_in_group_from_tree(node, ITEM_FACTORY_GROUP);
    factory.cast::<ItemFactory>()
}

pub(crate) fn get_enemy_factory_node_from_tree(node: &Gd<Node>) -> Gd<EnemyFactory> {
    let factory = get_node_in_group_from_tree(node, ENEMY_FACTORY_GROUP);
    factory.cast::<EnemyFactory>()
}

pub(crate) fn get_battle_engine_node_from_tree(node: &Gd<Node>) -> Gd<BattleEngine> {
    let engine = get_node_in_group_from_tree(node, BATTLE_ENGINE_GROUP);
    engine.cast::<BattleEngine>()
}

pub(crate) fn get_battle_setup_node_from_tree(node: &Gd<Node>) -> Gd<BattleSetup> {
    let setup = get_node_in_group_from_tree(node, BATTLE_SETUP_GROUP);
    setup.cast::<BattleSetup>()
}

pub(crate) fn get_board_node_from_tree(node: &Gd<Node>) -> Gd<Board> {
    let board = get_node_in_group_from_tree(node, BOARD_GROUP);
    board.cast::<Board>()
}

pub(crate) fn get_skill_factory_node_from_tree(node: &Gd<Node>) -> Gd<SkillFactory> {
    let board = get_node_in_group_from_tree(node, SKILL_FACTORY_GROUP);
    board.cast::<SkillFactory>()
}

pub(crate) fn get_skill_animation_factory_node_from_tree(
    node: &Gd<Node>,
) -> Gd<SkillAnimationFactory> {
    let board = get_node_in_group_from_tree(node, SKILL_ANIMATION_FACTORY_GROUP);
    board.cast::<SkillAnimationFactory>()
}

pub(crate) fn get_battle_entity_container_node_from_tree(
    node: &Gd<Node>,
) -> Gd<BattleEntityContainer> {
    let container = get_node_in_group_from_tree(node, BATTLE_ENTITY_CONTAINER_GROUP);
    container.cast::<BattleEntityContainer>()
}

pub(crate) fn get_summon_factory_node_from_tree(
    node: &Gd<Node>,
) -> Gd<SummonFactory> {
    let factory = get_node_in_group_from_tree(node, SUMMON_FACTORY_GROUP);
    factory.cast::<SummonFactory>()
}
