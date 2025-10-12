use std::collections::HashMap;

use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::skill::resource::manager::SkillResourceManager;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct CooldownTracker {
    #[export]
    skills_container: OnEditor<Gd<SkillContainerModule>>,
    // An alternative is HashMap<SkillDefinition, u8> if we don't mind assuming that an entity 
    // can't have more than one instance of a given skill. Or if so, sharing cooldown.
    cooldowns_tracking_table: HashMap<Gd<Skill>, u8>, 
    base: Base<Node>,
}

#[godot_dyn]
impl SkillResourceManager for CooldownTracker {
    
    fn consume_resources_for_casting(&mut self, skill: SkillDefinition) {
        let skill_node_to_track = self.get_skills_container().clone().unwrap().bind().get_skill_with_name(skill).unwrap();
        let definition = SkillDefinition::from_gstring(skill_node_to_track.bind().get_name());
        let skill_cooldown = definition.get_cooldown();
        let _previous_value = self.cooldowns_tracking_table.insert(skill_node_to_track, skill_cooldown);
    }

    fn has_resources_to_cast(&mut self, skill: SkillDefinition) -> bool {
        let skill_node_to_track = self.get_skills_container().clone().unwrap().bind().get_skill_with_name(skill).unwrap();
        if !self.cooldowns_tracking_table.contains_key(&skill_node_to_track) {
            return true;
        }
        let remaining_cooldown = self.cooldowns_tracking_table.get(&skill_node_to_track).unwrap();

        *remaining_cooldown <= 0
    }
}
