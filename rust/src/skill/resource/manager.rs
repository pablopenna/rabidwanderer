use crate::skill::skill_definition::SkillDefinition;

pub(crate) trait SkillResourceManager {
    fn consume_resources_for_casting(&mut self, skill: SkillDefinition);

    fn has_resources_to_cast(&self, skill: SkillDefinition) -> bool;

    /** mana cost or skill cooldown */
    fn get_resource_cost_for(&self, skill: SkillDefinition) -> u16;

    /** remaining mana or remaining skill cooldown */
    fn get_available_resource_for(&self, skill: SkillDefinition) -> u16;
}
