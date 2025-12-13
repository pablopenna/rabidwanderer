use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::SKILL_FACTORY_GROUP;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub(crate) struct SkillFactory {
    #[export]
    idle_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    tackle_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    bite_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    sonic_punch_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    lick_wounds_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    summon_spirit_scene: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillFactory {

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
            SkillDefinition::Idle => self.get_idle_scene().unwrap(),
            SkillDefinition::Tackle => self.get_tackle_scene().unwrap(),
            SkillDefinition::Bite => self.get_bite_scene().unwrap(),
            SkillDefinition::SonicPunch => self.get_sonic_punch_scene().unwrap(),
            SkillDefinition::LickWounds => self.get_lick_wounds_scene().unwrap(),
            SkillDefinition::SummonSpirit => self.get_summon_spirit_scene().unwrap(),
        }
    }
}
