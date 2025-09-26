use godot::classes::*;
use godot::prelude::*;

use crate::entity::board_entity::BoardEntity;
use crate::skill::skill::Skill;
use crate::utils::get_first_child_of_type::get_first_child_of_type;

const MAX_NUMBER_OF_SKILLS: i32 = 4;

// Items are placed as children of this node
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SkillContainerModule {
    base: Base<Node>,
}

#[godot_api]
impl SkillContainerModule {
    #[signal]
    pub(crate) fn skill_added(skill: Gd<Skill>);

    pub(crate) fn add_skill(&mut self, skill: Gd<Skill>) {
        let item_node = skill.clone().upcast::<Node>();
        self.base_mut().add_child(&item_node);
        self.signals().skill_added().emit(&skill);
    }

    pub(crate) fn has_room(&self) -> bool {
        self.base().get_child_count() < MAX_NUMBER_OF_SKILLS
    }

    pub(crate) fn get_skills(&self) -> Array<Gd<Skill>> {
        self.base().get_children().iter_shared().map(|child| child.cast::<Skill>()).collect()
    }

    pub(crate) fn get_skill_at(&self, index: usize) -> Gd<Skill> {
        self.base().get_children().get(index).unwrap().cast::<Skill>()
    }

    pub(crate) fn get_number_of_skills(&self) -> usize {
        self.base().get_children().len()
    }

    pub(crate) fn _remove_skill(&mut self, index: usize) {
        let children = self.base().get_children();
        let child_to_remove = children.get(index);
        if child_to_remove.is_some() {
            let child_to_remove = child_to_remove.unwrap();
            self.base_mut().remove_child(&child_to_remove);
        }
    }

    fn _get_skill_container_module_from(entity: Gd<BoardEntity>) -> Option<Gd<SkillContainerModule>> {
        let node = entity.upcast::<Node>();
        get_first_child_of_type::<SkillContainerModule>(&node)
    }
}
