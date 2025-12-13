use godot::obj::Gd;

use crate::entity::modules::skill::{
    skill_container::SkillContainerModule, skill_resource::SkillResourceModule,
};

/** Returns false if there are not enough resources for casting any of the skills */
pub(crate) fn can_cast_at_least_one_skill(
    skill_pool: &Gd<SkillContainerModule>,
    skill_resource: &Gd<SkillResourceModule>,
) -> bool {
    let all_skills = skill_pool.bind().get_skills();
    all_skills.iter_shared().any(|skill| {
        skill_resource
            .bind()
            .has_resources_to_cast(skill.bind().get_definition())
    })
}
