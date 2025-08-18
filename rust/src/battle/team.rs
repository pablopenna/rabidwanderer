use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub enum Team {
    #[default]
    Enemy,
    Player,
}

impl Team {
    pub(crate) fn get_entities_from_team(team: Team, entities: &Array<Gd<BattleEntity>>) -> Array<Gd<BattleEntity>> {
        entities.iter_shared().filter(
            |e| team == Team::from_variant(&e.bind().get_team().to_variant())
        ).collect()
    }

    pub(crate) fn are_there_entities_from_team(team: Team, entities: &Array<Gd<BattleEntity>>) -> bool {
        let entities = Team::get_entities_from_team(team, entities);
        entities.len() > 0
    }
}
