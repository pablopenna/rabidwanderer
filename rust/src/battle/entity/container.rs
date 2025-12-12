use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;
use crate::consts::groups::BATTLE_ENTITY_CONTAINER_GROUP;

/*
 * BEs (Battle Entities) are stored as children instead of as references in an array.
 * While the latter would be better in theory, we need to store them as children because
 * the battle node tree is placed within a subviewport and thus in a different World2D.
 *
 * Apparently, you cannot change a CanvasItem World2D. That is just decided from where they are on the tree.
 * We need them to be in the battle World2D so they can be displayed in the Battle UI window.
 */
#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct BattleEntityContainer {
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleEntityContainer {
    fn ready(&mut self) {
        self.base_mut().add_to_group(BATTLE_ENTITY_CONTAINER_GROUP);
    }
}

#[godot_api]
impl BattleEntityContainer {
    pub(crate) fn get_entity_count(&self) -> i32 {
        self.base().get_child_count()
    }

    pub(crate) fn get_all_entities(&self) -> Array<Gd<BattleEntity>> {
        self.base()
            .get_children()
            .iter_shared()
            .map(|child| child.cast::<BattleEntity>())
            .collect()
    }

    pub(crate) fn get_entities_from_team(&self, team: &Team) -> Array<Gd<BattleEntity>> {
        let all_entities = self.get_all_entities();

        Team::get_entities_from_team(team, &all_entities)
    }

    pub(crate) fn get_alive_entities_from_team(&self, team: &Team) -> Array<Gd<BattleEntity>> {
        let entities_from_team = self.get_entities_from_team(team);

        entities_from_team
            .iter_shared()
            .filter(|entity| entity.bind().get_stats().bind().is_alive())
            .collect()
    }

    pub(crate) fn get_alive_entities(&self) -> Array<Gd<BattleEntity>> {
        let all_entities = self.get_all_entities();

        all_entities
            .iter_shared()
            .filter(|entity| entity.bind().get_stats().bind().is_alive())
            .collect()
    }

    pub(crate) fn add_entity(&mut self, entity: &Gd<BattleEntity>) {
        self.base_mut().add_child(entity);
    }

    /** Removes the entities as children but does not free them so they are still in memory */
    pub(crate) fn remove_all_entities(&mut self) {
        let children = self.base_mut().get_children();
        children
            .iter_shared()
            .for_each(|child| self.base_mut().remove_child(&child));
    }
}
