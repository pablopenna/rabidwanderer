use godot::classes::object::ConnectFlags;
use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::container::BattleEntityContainer;
use crate::battle::team::Team;
use crate::consts::groups::get_battle_entity_container_node_from_tree;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::chooser::skill_chooser::SkillChooser;
use crate::skill::skill_definition::SkillDefinition;
use crate::skill::skill_implementation::SkillImplementation;
use crate::stats::real::RealStats;
use crate::stats::stat::Stat;
use crate::targeting::target_amount::TargetAmount;
use crate::targeting::target_faction::TargetFaction;

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
    animation_player: OnEditor<Gd<AnimationPlayer>>,
    #[export]
    sprite: OnEditor<Gd<Sprite2D>>,
    battle_entity_container: Option<Gd<BattleEntityContainer>>,
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
            animation_player: OnEditor::default(),
            sprite: OnEditor::default(),
            battle_entity_container: None,
        }
    }
}

#[godot_api]
impl BattleEntity {
    #[signal]
    pub(crate) fn death();

    #[signal]
    pub(crate) fn skill_chosen(skill_name: SkillDefinition, targets: Array<Gd<BattleEntity>>);
    
    #[signal]
    pub(crate) fn done_acting();

    pub(crate) fn get_entity_team(&self) -> Team {
        Team::from_gstring(self.team.get_property())
    }

    pub(crate) fn take_damage(&mut self, damage: u16) {
        godot_print!("[{}] is taking {} damage", self.base().get_name(), damage);
        let hp = self.stats.bind_mut().get_current_hp();
        if hp > damage {
            self.stats.bind_mut().set_current_hp(hp - damage);
        } else {
            self.stats.bind_mut().set_current_hp(0);
            self.signals().death().emit();
        }
        godot_print!(
            "[{}] Ouch! I have {} remaining",
            self.base().get_name(),
            self.stats.bind().get_current_hp()
        );
    }

    #[func(gd_self)]
    pub(crate) fn choose_skill(this: Gd<Self>) {
        let skills_container = this.bind().get_skills();
        let skill_chooser = this.bind().get_skill_chooser().unwrap();
        let skill_resource = this.bind().get_skill_resource();

        let container = BattleEntity::get_battle_entity_container(this.clone());
        let target_candidates = container.bind().get_all_entities();

        // Ugly but needed AFAIK due to borrowing and lifecycle
        let that = this.clone();

        // Passing the "this" variable as parameter to connect_other gives compilation error
        // Workaround is using connect() with a closure + "move" keyword. Source:
        // https://github.com/godot-rust/gdext/issues/1318#issuecomment-3306720324
        skill_chooser
            .signals()
            .skill_chosen()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect(
                move |skill_name,
                      skill_implementation,
                      skill_resource_from_signal,
                      targets,
                      target_amount,
                      target_faction| {
                    Self::on_skill_chosen(
                        that.clone(),
                        skill_name,
                        skill_implementation,
                        skill_resource_from_signal,
                        targets,
                        target_amount,
                        target_faction,
                    )
                },
            );

        skill_chooser.signals().choose_skill().emit(
            &skills_container,
            &skill_resource,
            &this,
            &target_candidates,
        );
    }

    #[func(gd_self)]
    fn on_skill_chosen(
        this: Gd<Self>,
        skill_name: SkillDefinition,
        mut _skill: DynGd<Node, dyn SkillImplementation>,
        mut _skill_resource: Gd<SkillResourceModule>,
        targets: Array<Gd<BattleEntity>>,
        _target_amount: TargetAmount,
        _target_faction: TargetFaction,
    ) {
        this.signals().skill_chosen().emit(skill_name, &targets);
    }
    
    #[func(gd_self)]
    pub(crate) fn cast_skill(
        this: Gd<Self>,
        skill_name: SkillDefinition,
        targets: Array<Gd<BattleEntity>>,
    ) {
        let mut skill_node = this.bind().skills.bind().get_skill_with_name(skill_name).unwrap();
        let mut skill_implementation = skill_node.bind_mut().get_implementation();
        
        skill_implementation.dyn_bind_mut().cast(this.clone(), &targets);
    }

    #[func]
    fn on_skill_casting_done(&mut self) {
        if self.can_cast_more_skills() {
            Self::choose_skill(self.to_gd());
            return;
        }
        self.on_done_acting();
    }

    // TODO: move to skill (as targets are visible to skill)
    #[func(gd_self)]
    fn on_apply_damage(this: Gd<Self>, targets: Array<Gd<BattleEntity>>) {
        godot_print!("[{}] I'm attacking", this.get_name());
        let attack_damage = this.bind().get_stats().bind().get_stat(Stat::Attack);
        godot_print!(
            "[{}] I am about to deal {} damage",
            this.get_name(),
            attack_damage
        );

        targets.iter_shared().for_each(|mut target| {
            godot_print!(
                "[{}] I am about to attack {}",
                this.get_name(),
                target.get_name()
            );

            target.bind_mut().take_damage(attack_damage.max(0) as u16);
        });
    }

    #[func]
    fn can_cast_more_skills(&self) -> bool {
        false
    }

    #[func]
    fn on_done_acting(&mut self) {
        godot_print!("[{}] I'm done", self.base().get_name());
        self.signals().done_acting().emit();
    }

    #[func(gd_self)]
    fn get_battle_entity_container(mut this: Gd<Self>) -> Gd<BattleEntityContainer> {
        if this.bind().battle_entity_container.is_none() {
            let node = this.clone().upcast::<Node>();
            this.bind_mut().battle_entity_container =
                Some(get_battle_entity_container_node_from_tree(&node));
        }

        this.bind().battle_entity_container.clone().unwrap()
    }
}
