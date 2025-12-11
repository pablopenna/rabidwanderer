use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_implementation::SkillImplementation;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_faction::TargetFaction;

// pub(crate) trait SkillChooser {
//     fn choose(&mut self, skill_pool: &Gd<SkillContainerModule>, target: &Gd<BattleEntity>) -> DynGd<Node, dyn SkillImplementation>;
// }

// The choosing of a skill can be async (when player is choosing via UI).
// To accomodate for this, we need to make the process of choosing a skill async.
// A way of doing it is emitting a signal when the choice is done.
// As of 0.4.3 we cannot define signals within traits.

// To work around that we define
// a parent SkillChooser node that acts like the interface to a child that
// implements the logic to choose a skill.

#[derive(GodotClass)]
#[class(base=Node,init)]
pub(crate) struct SkillChooser {
    base: Base<Node>,
}

#[godot_api]
impl SkillChooser {
    // To be emited by battle entity to get a skill chosen. It is listened by the child node that implements the logic
    #[signal]
    pub(crate) fn choose_skill(
        skill_pool: Gd<SkillContainerModule>,
        skill_resource: Gd<SkillResourceModule>,
        actor: Gd<BattleEntity>,
        target_candidates: Array<Gd<BattleEntity>>,
    );

    // emited by the child of this node once a skill is chosen. The battle entity emiting the signal above should listen to this
    #[signal]
    pub(crate) fn skill_chosen(
        skill_name: SkillDefinition,
        skill_implementation: DynGd<Node, dyn SkillImplementation>,
        skill_priority: i32,
        skill_resource: Gd<SkillResourceModule>,
        targets: Array<Gd<BattleEntity>>,
        target_amount: TargetAmount,
        target_faction: TargetFaction,
    );
}
