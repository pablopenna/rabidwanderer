use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;

/**
 * Used to place BattleEntities **of a given team**
 * in the BattleArena.
 */
#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub(crate) struct BattleEntityPlacer {
    base: Base<Node2D>,
    #[export]
    container: OnEditor<Gd<BattleEntityContainer>>,
    #[export]
    player_team_reference_position: OnEditor<Gd<Node2D>>,
    #[export]
    enemy_team_reference_position: OnEditor<Gd<Node2D>>,
    #[export]
    offset: f32,
}

#[godot_api]
impl BattleEntityPlacer {
    /**
     * Should be called before adding the entity to the BattleEntityContainer
     */
    // pub(crate) fn place_entity(&self, mut entity: Gd<BattleEntity>) {
    //     let team = entity.bind().get_entity_team();

    //     let position = self.get_target_position_for_team(&team);
    //     entity.set_global_position(position);
    // }

    pub(crate) fn place_entity_at_index(&self, mut entity: Gd<BattleEntity>, index: usize) {
        let team = entity.bind().get_entity_team();

        let position = self.get_position_at_index_for_team(&team, index);
        entity.set_global_position(position);
    }

    // fn get_target_position_for_team(&self, team: &Team) -> Vector2 {
    //     let container = self.get_container().unwrap();
    //     let amount_of_existing_entities_from_that_team =
    //         container.bind().get_entities_from_team(team).len();
    //     let target_position =
    //         self.get_position_at_index_for_team(team, amount_of_existing_entities_from_that_team);

    //     target_position
    // }

    fn get_position_at_index_for_team(&self, team: &Team, index: usize) -> Vector2 {
        let initial_position = self.get_initial_position_for_team(team);
        let direction = self.get_placing_direction_for_team(team);

        let mut position = initial_position.clone();
        for i in 1..index + 1 {
            if i == 1 {
                position += Vector2 {
                    x: direction.x * self.offset,
                    y: -self.offset * 0.5,
                }
            } else if i % 2 == 0 {
                position += Vector2 {
                    x: 0f32,
                    y: self.offset,
                }
            } else {
                position += Vector2 {
                    x: direction.x * self.offset,
                    y: -self.offset,
                }
            }
        }
        position
    }

    fn get_initial_position_for_team(&self, team: &Team) -> Vector2 {
        match team {
            Team::Player => self
                .get_player_team_reference_position()
                .unwrap()
                .get_global_position(),
            Team::Enemy => self
                .get_enemy_team_reference_position()
                .unwrap()
                .get_global_position(),
        }
    }

    fn get_placing_direction_for_team(&self, team: &Team) -> Vector2 {
        match team {
            Team::Player => Vector2::LEFT,
            Team::Enemy => Vector2::RIGHT,
        }
    }
}
