use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::skill::skill_container::SkillContainerModule;
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

    pub(crate) fn act(&mut self) {
        // Once animations are added, these methods should be called from the animation.
        // This method then would only kickstart the animation
        // self.on_apply_damage();
        // self.on_done_acting();

        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_property(
            &self.sprite.get_property().unwrap(),
            "scale", 
            &Vector2{x: 2.0, y: 2.0}.to_variant(), 
            1.0
        );
        // tween.parallel().unwrap().tween_property(
        //     &self.sprite.get_property().unwrap(),
        //     "frame", 
        //     &Variant::from(3),
        //     1.0
        // );
        tween.tween_property(
            &self.sprite.get_property().unwrap(),
            "scale", 
            &Vector2{x: 1.0, y: 1.0}.to_variant(), 
            1.0
        );
        // tween.parallel().unwrap().tween_property(
        //     &self.sprite.get_property().unwrap(),
        //     "frame", 
        //     &Variant::from(0), 
        //     1.0
        // );
        tween.tween_callback(&Callable::from_object_method(&self.to_gd(), "on_apply_damage"));
        tween.tween_callback(&Callable::from_object_method(&self.to_gd(), "on_done_acting")).unwrap().set_delay(1.0);

        //self.get_animation_player().unwrap().play_ex().name("attack").done();
    }

    #[func(gd_self)]
    pub(crate) fn act_with_skill_hardcoded(mut this: Gd<Self>) {
        let mut skill = get_skill_implementation(SkillDefinition::Bite);
        this.add_child(&skill);
        // let skill_callable = Callable::from_object_method(&self.to_gd(), "cast_skill");
        // let skill_callable_with_args = skill_callable.bind(&[skill.to_variant()]);
        
        skill.dyn_bind_mut().cast(this.clone(), this.bind().target.clone().unwrap());
    }

    #[func(gd_self)]
    pub(crate) fn act_with_skill(mut this: Gd<Self>) {
        let skill_definition = this.bind().get_skills().bind().get_skills().at(0).bind().get_name();
        let mut skill = get_skill_implementation(SkillDefinition::from_gstring(skill_definition));
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
