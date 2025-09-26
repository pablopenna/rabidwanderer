use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::get_implementation::get_skill_implementation;
use crate::skill::skill_definition::SkillDefinition;
use crate::stats::real::RealStats;
use crate::battle::team::Team;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub(crate) struct BattleEntity {
    base: Base<Node2D>,
    #[var]
    stats: Gd<RealStats>, // Stats are passed from Board Entity on instantiation so that they are preserved in between battles
    #[var]
    skills: Gd<SkillContainerModule>, // Same as stats
    #[export]
    skill_chooser: OnEditor<DynGd<Node, dyn SkillChooser>>,
    #[export]
    team: Team,
    #[export]
    target: Option<Gd<BattleEntity>>,
    #[export]
    animation_player: OnEditor<Gd<AnimationPlayer>>,
    #[export]
    sprite: OnEditor<Gd<Sprite2D>>,
}

#[godot_api]
impl INode2D for BattleEntity {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            stats: RealStats::new_gd(),
            skills: SkillContainerModule::new_alloc(),
            skill_chooser: OnEditor::default(),
            team: Team::Player,
            target: None,
            animation_player: OnEditor::default(),
            sprite: OnEditor::default(),
        }
    }
}

#[godot_api]
impl BattleEntity {
    #[signal]
    pub(crate) fn death();

    #[signal]
    pub(crate) fn done_acting();

    pub(crate) fn take_damage(&mut self, damage: u16) {
        let hp = self.stats.bind_mut().get_current_hp();
        if hp > damage {
            self.stats.bind_mut().set_current_hp(hp - damage);
        } else {
            self.stats.bind_mut().set_current_hp(0);
            self.signals().death().emit();
        }
        godot_print!("Ouch! I have {} remaining", self.stats.bind().get_current_hp());
    }

    #[func(gd_self)]
    pub(crate) fn act_with_skill(mut this: Gd<Self>) {
        let skills_container = this.bind().get_skills();
        let target = this.bind().get_target().unwrap();
        let skill_chooser_node = this.bind().get_skill_chooser().unwrap();
        
        let mut skill_chooser: DynGd<Node, dyn SkillChooser> = skill_chooser_node.to_variant().to();
        let skill_definition = skill_chooser.dyn_bind_mut().choose(&skills_container, &target);
        let mut skill = get_skill_implementation(skill_definition);
        this.add_child(&skill);
        // let skill_callable = Callable::from_object_method(&self.to_gd(), "cast_skill");
        // let skill_callable_with_args = skill_callable.bind(&[skill.to_variant()]);
        
        skill.dyn_bind_mut().cast(this.clone(), this.bind().target.clone().unwrap());
    }

    #[func]
    fn on_apply_damage(&mut self) {
        godot_print!("I am {} and I'm attacking", self.base().get_name());
        let attack_damage = self.stats.bind().calculate_attack_damage();
        self.target.clone().unwrap().bind_mut().take_damage(attack_damage);
    }

    #[func]
    fn on_done_acting(&mut self) {
        godot_print!("I am {} and I'm done", self.base().get_name());
        self.signals().done_acting().emit();
    }
}
