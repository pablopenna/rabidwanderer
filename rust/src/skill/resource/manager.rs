use crate::skill::skill_definition::SkillDefinition;

pub(crate) trait SkillResourceManager {
    fn consume_resources_for_casting(&mut self, skill: SkillDefinition);

    fn has_resources_to_cast(&mut self, skill: SkillDefinition) -> bool;
}
