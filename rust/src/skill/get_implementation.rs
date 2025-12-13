use godot::classes::Node;
use godot::obj::{DynGd, NewAlloc};

use crate::skill::implementations::bite::BiteSkillImplementation;
use crate::skill::implementations::idle::IdleSkillImplementation;
use crate::skill::implementations::lick_wounds::LickWoundsSkillImplementation;
use crate::skill::implementations::sonic_punch::SonicPunchSkillImplementation;
use crate::skill::implementations::summon_spirit::SummonSpiritSkillImplementation;
use crate::skill::implementations::tackle::TackleSkillImplementation;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_implementation::SkillImplementation;

// https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.DynGd.html
// Important: caller should add node returned to the tree. Not doing that will cause undesired behaviours in the game
pub(crate) fn get_skill_implementation(skill_name: SkillDefinition) -> DynGd<Node, dyn SkillImplementation> {
    match skill_name {
        SkillDefinition::Idle => IdleSkillImplementation::new_alloc().into_dyn::<dyn SkillImplementation>().upcast::<Node>(),
        SkillDefinition::Tackle => TackleSkillImplementation::new_alloc().into_dyn::<dyn SkillImplementation>().upcast::<Node>(),
        SkillDefinition::Bite => BiteSkillImplementation::new_alloc().into_dyn::<dyn SkillImplementation>().upcast::<Node>(),
        SkillDefinition::SonicPunch => SonicPunchSkillImplementation::new_alloc().into_dyn::<dyn SkillImplementation>().upcast::<Node>(),
        SkillDefinition::LickWounds => LickWoundsSkillImplementation::new_alloc().into_dyn::<dyn SkillImplementation>().upcast::<Node>(),
        SkillDefinition::SummonSpirit => SummonSpiritSkillImplementation::new_alloc().into_dyn::<dyn SkillImplementation>().upcast::<Node>(),
    }
}
