use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;

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
impl BattleEntityContainer {
    pub(crate) fn get_entity_count(&self) -> i32 {
        self.base().get_child_count()
    }

    pub(crate) fn get_all_entities(&self) -> Array<Gd<BattleEntity>> {
        self.base().get_children().iter_shared().map(|child| child.cast::<BattleEntity>()).collect()
    }

    pub(crate) fn get_entities_from_team(&self, team: Team) -> Array<Gd<BattleEntity>> {
        self.base().get_children().iter_shared()
        .map(|child| child.cast::<BattleEntity>())
        .filter(|entity| entity.bind().get_team() == team.get_property())
        .collect()
    }

    pub(crate) fn get_alive_entities(&self) -> Array<Gd<BattleEntity>> {
        self.base().get_children().iter_shared()
        .map(|child| child.cast::<BattleEntity>())
        .filter(|entity| entity.bind().get_stats().bind().is_alive())
        .collect()
    }

    pub(crate) fn add_entity(& mut self, entity: &Gd<BattleEntity>) {
        self.base_mut().add_child(entity);
    }

    /** Removes the entities as children but does not free them so they are still in memory */
    pub(crate) fn remove_all_entities(&mut self) {
        let children = self.base_mut().get_children();
        children.iter_shared().for_each(
            |child| self.base_mut().remove_child(&child)
        );
    }
}


