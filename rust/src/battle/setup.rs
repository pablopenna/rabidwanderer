use godot::classes::*;
use godot::prelude::*;

use crate::battle::engine::BattleEngine;
use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;
use crate::board::board::Board;
use crate::board::coordinate::BoardCoordinate;
use crate::consts::groups::get_board_node_from_tree;
use crate::consts::groups::BATTLE_SETUP_GROUP;
use crate::entity::modules::battle::battle::BattleModule;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct BattleSetup {
    base: Base<Node>,
    #[export]
    player_position_reference: OnEditor<Gd<Node2D>>,
    #[export]
    enemy_position_reference: OnEditor<Gd<Node2D>>,
    #[export]
    container: OnEditor<Gd<BattleEntityContainer>>,
    #[export]
    engine: OnEditor<Gd<BattleEngine>>,
    board: Option<Gd<Board>>,

}

#[godot_api]
impl INode for BattleSetup {
    fn ready(&mut self) {
        self.base_mut().add_to_group(BATTLE_SETUP_GROUP);
    }
}

#[godot_api]
impl BattleSetup {
    pub(crate) fn setup_combat_for_tile(&mut self, coord: &BoardCoordinate) {
        let tile = self.get_board().bind().get_tile_at(&coord).unwrap();
        let entities = tile.bind().get_entities();
        
        let battle_modules: Array<Gd<BattleModule>> = entities.iter_shared()
            .map(|e| BattleModule::get_battle_module_from_entity(e))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .collect();
        
        let scenes: Array<Gd<PackedScene>> = battle_modules.iter_shared()
            .map(|e| e.bind().get_battle_entity())
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .collect();
        
        let battle_entities: Array<Gd<BattleEntity>> = scenes.iter_shared()
            .map(|e| e.instantiate_as::<BattleEntity>())
            .collect();

        let are_there_enemies = Team::are_there_entities_from_team(Team::Enemy, &battle_entities);
        if !are_there_enemies { return; }
        
        battle_entities.iter_shared().for_each(
            |e| self.container.bind_mut().add_entity(&e)
        );
        //TODO: set targets for each battle_entity
    }

    fn get_board(&mut self) -> Gd<Board> {
        if self.board.is_none() {
            self.board = Some(get_board_node_from_tree(self.base().clone().upcast::<Node>()));
        }

        self.board.clone().unwrap()
    }
}


