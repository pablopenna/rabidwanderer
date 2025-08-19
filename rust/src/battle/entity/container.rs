use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::team::Team;

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
        .filter(|entity| entity.bind().get_stats().unwrap().bind().is_alive())
        .collect()
    }

    pub(crate) fn add_entity(& mut self, entity: &Gd<BattleEntity>) {
        self.base_mut().add_child(entity);
    }

    pub(crate) fn remove_all_entities(&mut self) {
        let children = self.base_mut().get_children();
        children.iter_shared().for_each(
            |child| self.base_mut().remove_child(&child)
        );
    }
}


