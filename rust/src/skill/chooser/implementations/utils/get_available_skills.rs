use godot::{classes::class_macros::private::virtuals::Os::Array, obj::Gd};

use crate::{
    entity::modules::skill::{
        skill_container::SkillContainerModule, skill_resource::SkillResourceModule,
    },
    skill::skill::Skill,
};

/** Gets the skills which it has enough resources to cast */
pub(crate) fn get_available_skills(
    skill_pool: &Gd<SkillContainerModule>,
    skill_resource: &Gd<SkillResourceModule>,
) -> Array<Gd<Skill>> {
    let all_skills = skill_pool.bind().get_skills();
    all_skills
        .iter_shared()
        .filter(|skill| {
            skill_resource
                .bind()
                .has_resources_to_cast(skill.bind().get_definition())
        })
        .collect()
}
