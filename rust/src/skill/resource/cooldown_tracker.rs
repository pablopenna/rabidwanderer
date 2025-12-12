use std::collections::HashMap;

use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::resource::manager::SkillResourceManager;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct CooldownTracker {
    #[export]
    skill_resource_module: OnEditor<Gd<SkillResourceModule>>,
    #[export]
    skills_container: OnEditor<Gd<SkillContainerModule>>,
    // An alternative is HashMap<SkillDefinition, u8> if we don't mind assuming that an entity
    // can't have more than one instance of a given skill. Or if so, sharing cooldown.
    cooldowns_tracking_table: HashMap<Gd<Skill>, u8>,
    base: Base<Node>,
}

#[godot_api]
impl INode for CooldownTracker {
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_dyn]
impl SkillResourceManager for CooldownTracker {
    fn consume_resources_for_casting(&mut self, skill: SkillDefinition) {
        let skill_node_to_track = self.get_skill_node_for_skill(skill).unwrap();
        let skill_cooldown = skill_node_to_track.bind().get_cooldown();
        let _previous_value = self
            .cooldowns_tracking_table
            .insert(skill_node_to_track, skill_cooldown);
    }

    fn has_resources_to_cast(&self, skill: SkillDefinition) -> bool {
        let skill_node_to_track = self.get_skill_node_for_skill(skill).unwrap();
        if !self
            .cooldowns_tracking_table
            .contains_key(&skill_node_to_track)
        {
            return true;
        }
        let remaining_cooldown = self
            .cooldowns_tracking_table
            .get(&skill_node_to_track)
            .unwrap();

        *remaining_cooldown <= 0
    }

    /* get skill cooldown */
    fn get_resource_cost_for(&self, skill: SkillDefinition) -> u16 {
        let skill_cooldown = skill.get_cooldown();
        skill_cooldown as u16
    }

    /* get remaining skill cooldown */
    fn get_available_resource_for(&self, skill: SkillDefinition) -> u16 {
        let skill_node = self.get_skill_node_for_skill(skill).unwrap();
        let remaining_cooldown = self.cooldowns_tracking_table.get(&skill_node);

        if remaining_cooldown.is_none() {
            return 0;
        }

        *remaining_cooldown.unwrap() as u16
    }
}

impl CooldownTracker {
    fn setup(&mut self) {
        self.skill_resource_module
            .signals()
            .added_to_battle_entity()
            .connect_other(self, Self::on_added_to_battle_entity);
    }

    fn on_added_to_battle_entity(&mut self, battle_entity: Gd<BattleEntity>) {
        battle_entity
            .signals()
            .done_acting()
            .connect_other(self, Self::on_battle_entity_acted);

        //TODO: remove connection when battle entity is freed (should be doable via signal)
    }

    fn on_battle_entity_acted(&mut self) {
        godot_print!("refreshing cooldowns...");
        self.refresh_cooldowns();
        godot_print!("refreshing cooldowns done.");
    }

    fn refresh_cooldowns(&mut self) {
        let keys: Vec<Gd<Skill>> = self.cooldowns_tracking_table.keys().cloned().collect();
        for skill in keys {
            self.refresh_single_cooldown(skill);
        }
    }

    fn refresh_single_cooldown(&mut self, skill: Gd<Skill>) {
        let cooldown = self.cooldowns_tracking_table.get_mut(&skill).unwrap();
        if *cooldown > 0 {
            *cooldown -= 1;
        }
    }

    fn get_skill_node_for_skill(&self, skill: SkillDefinition) -> Option<Gd<Skill>> {
        self.get_skills_container()
            .clone()
            .unwrap()
            .bind()
            .get_skill_with_name(skill)
    }
}
