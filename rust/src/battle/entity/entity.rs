use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_implementation::SkillImplementation;
use crate::stats::real::RealStats;
use crate::battle::team::Team;
use crate::stats::stat::Stat;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub(crate) struct BattleEntity {
    base: Base<Node2D>,
    #[var]
    stats: Gd<RealStats>, // Stats are passed from Board Entity on instantiation so that they are preserved in between battles
    #[var]
    skills: Gd<SkillContainerModule>, // Same as stats
    #[export]
    skill_chooser: OnEditor<Gd<SkillChooser>>,
    #[var]
    skill_resource: Gd<SkillResourceModule>, // Same as stats
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
            skill_resource: SkillResourceModule::new_alloc(),
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
    pub(crate) fn act_with_skill(this: Gd<Self>) {
        let skills_container = this.bind().get_skills();
        let target = this.bind().get_target().unwrap();
        let skill_chooser = this.bind().get_skill_chooser().unwrap();
        
        // Passing the "this" variable as parameter to connect_other gives compilation error
        // Workaround is using connect() with a closure + "move" keyword. Source:
        // https://github.com/godot-rust/gdext/issues/1318#issuecomment-3306720324
        skill_chooser.signals().skill_chosen().builder()
        .flags(ConnectFlags::ONE_SHOT)
        .connect( move |skill_name, skill_implementation| 
            Self::on_skill_chosen(this.clone(), skill_name, skill_implementation)
        );
        skill_chooser.signals().choose_skill().emit(&skills_container, &target);
    }

    #[func(gd_self)]
    fn on_skill_chosen(mut this: Gd<Self>, skill_name: SkillDefinition, mut skill: DynGd<Node, dyn SkillImplementation>) {
        let skill_resource = this.bind().get_skill_resource();
        if !skill_resource.bind().has_resources_to_cast(skill_name.clone()) {
            // TODO: do not lose action
            godot_print!("[{}] resources to cast {} are not enough", this.get_name(), skill_name.to_variant());
            this.bind_mut().on_done_acting();
            return;
        }
        skill_resource.bind().consume_resources_for_casting(skill_name);
        skill.dyn_bind_mut().cast(this.clone(), this.bind().target.clone().unwrap());
    }

    #[func]
    fn on_skill_casting_done(&mut self) {
        if self.can_cast_more_skills() {
            Self::act_with_skill(self.to_gd());
            return;
        }
        self.on_done_acting();
    }

    #[func]
    fn on_apply_damage(&mut self) {
        godot_print!("I am {} and I'm attacking", self.base().get_name());
        let attack_damage = self.stats.bind().get_stat(Stat::Attack);
        self.target.clone().unwrap().bind_mut().take_damage(attack_damage.max(0) as u16);
    }

    #[func]
    fn can_cast_more_skills(&self) -> bool {
        false
    }

    #[func]
    fn on_done_acting(&mut self) {
        godot_print!("I am {} and I'm done", self.base().get_name());
        self.signals().done_acting().emit();
    }
}
