use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::SKILL_FACTORY_GROUP;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct SkillFactory {
    #[export]
    tackle_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    bite_scene: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillFactory {
    fn init(base: Base<Node>) -> Self {
        Self {
            tackle_scene: OnEditor::default(),
            bite_scene: OnEditor::default(),
            base
        }
    }

    fn ready(&mut self) {
        let mut node = self.base_mut().to_godot().clone().upcast::<Node>();
        node.add_to_group(SKILL_FACTORY_GROUP);
    }
}

impl SkillFactory {
    pub(crate) fn instance_skill(&self, def: &SkillDefinition) -> Gd<Skill> {
        let scene = self.get_scene_for_skill(def);
        let item = scene.instantiate_as::<Skill>();
        item
    }

    fn get_scene_for_skill(&self, def: &SkillDefinition) -> Gd<PackedScene> {
        match def {
            SkillDefinition::Tackle => self.get_tackle_scene().unwrap(),
            SkillDefinition::Bite => self.get_bite_scene().unwrap(),
        }
    }
}
