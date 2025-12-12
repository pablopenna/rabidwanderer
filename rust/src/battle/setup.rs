use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::entity::entity::BattleEntity;
use crate::battle::entity::placer::BattleEntityPlacer;
use crate::battle::team::Team;
use crate::battle::turns::turn_handler::TurnHandler;
use crate::board::board::Board;
use crate::board::coordinate::BoardCoordinate;
use crate::consts::groups::get_board_node_from_tree;
use crate::consts::groups::BATTLE_SETUP_GROUP;
use crate::entity::modules::battle::battle::BattleModule;
use crate::global_signals::GlobalSignals;

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
    placer: OnEditor<Gd<BattleEntityPlacer>>,
    #[export]
    turns_handler: OnEditor<Gd<TurnHandler>>,
    board: Option<Gd<Board>>,
}

#[godot_api]
impl INode for BattleSetup {
    fn ready(&mut self) {
        self.base_mut().add_to_group(BATTLE_SETUP_GROUP);

        GlobalSignals::get_singleton()
            .signals()
            .battle_finished()
            .connect_other(self, Self::cleanup_combat);
    }
}

#[godot_api]
impl BattleSetup {
    pub(crate) fn setup_combat_for_tile(&mut self, coord: &BoardCoordinate) {
        let battle_entities = self.get_instances(coord);

        // This check is enough as this method should be called by the player each time it moves, meaning the player should always be among the entities
        let are_there_enemies = Team::are_there_entities_from_team(&Team::Enemy, &battle_entities);
        if !are_there_enemies {
            return;
        }

        self.add_instances_to_container(&battle_entities);
        self.set_instances_position_for_all_teams(&battle_entities);

        GlobalSignals::get_singleton()
            .signals()
            .battle_set_up()
            .emit();
    }
    
    pub(crate) fn cleanup_combat(&mut self) {
        self.container.clone().bind_mut().remove_all_entities();
        self.turns_handler
            .clone()
            .bind_mut()
            .remove_all_actions_from_turn();
    }
    
    pub(crate) fn add_entity_to_combat(&mut self, entity: Gd<BattleEntity>) {
        self.add_single_instance_to_container(&entity);
        self.set_single_instance_position(entity);
    }

    fn get_instances(&mut self, coord: &BoardCoordinate) -> Array<Gd<BattleEntity>> {
        let tile = self.get_board().bind().get_tile_at(&coord).unwrap();
        let entities = tile.bind().get_entities();

        let battle_modules: Array<Gd<BattleModule>> = entities
            .iter_shared()
            .map(|e| BattleModule::get_battle_module_from_entity(e))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .collect();

        let battle_entities: Array<Gd<BattleEntity>> = battle_modules
            .iter_shared()
            .map(|mut e| e.bind_mut().get_battle_entity_instance())
            .collect();

        battle_entities
    }

    fn set_instances_position_for_all_teams(&self, all_instances: &Array<Gd<BattleEntity>>) {
        let player_team = Team::get_entities_from_team(&Team::Player, all_instances);
        self.set_instances_position(&player_team);

        let enemy_team = Team::get_entities_from_team(&Team::Enemy, all_instances);
        self.set_instances_position(&enemy_team);
    }

    fn set_instances_position(&self, instances: &Array<Gd<BattleEntity>>) {
        for i in 0..instances.len() {
            let entity = instances.at(i);
            self.placer.bind().place_entity_at_index(entity, i);
        }
    }
    
    fn set_single_instance_position(&self, instance: Gd<BattleEntity>) {
        let team = instance.bind().get_entity_team();
        let amount_of_entities_in_team = self.container.bind().get_entities_from_team(&team).len();
        self.placer.bind().place_entity_at_index(instance, amount_of_entities_in_team);
    }

    fn add_instances_to_container(&mut self, instances: &Array<Gd<BattleEntity>>) {
        instances
            .iter_shared()
            .for_each(|e| self.add_single_instance_to_container(&e));
    }
 
    fn add_single_instance_to_container(&mut self, instance: &Gd<BattleEntity>) {
        self.container.bind_mut().add_entity(instance);
    }

    fn get_board(&mut self) -> Gd<Board> {
        if self.board.is_none() {
            self.board = Some(get_board_node_from_tree(&self.base()));
        }

        self.board.clone().unwrap()
    }
}
