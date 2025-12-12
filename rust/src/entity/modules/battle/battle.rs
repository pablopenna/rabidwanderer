use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::battle::ui::hp_bar::HpBar;
use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::entity::modules::stats::stats::StatsModule;
use crate::stats::real::RealStats;
use crate::utils::get_first_child_of_type::get_first_child_of_type;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleModule {
    #[export]
    battle_entity_scene: OnEditor<Gd<PackedScene>>,
    battle_entity_instance: OnReady<Gd<BattleEntity>>,
    #[export]
    stats: OnEditor<Gd<StatsModule>>, // IMPORTANT: StatsModule needs to be a child previous to this module so its ready is called first. If not, an OnReady error will be thrown 
    #[export]
    skills: OnEditor<Gd<SkillContainerModule>>, // IMPORTANT: SkillCOntainerModule needs to be a previous child so its ready is called before this one
    #[export]
    skill_resources: OnEditor<Gd<SkillResourceModule>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for BattleModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            battle_entity_scene: OnEditor::default(),
            battle_entity_instance: OnReady::manual(),
            stats: OnEditor::default(),
            skills: OnEditor::default(),
            skill_resources: OnEditor::default(),
            base,
        }
    }

    fn ready(&mut self) {
        let instance = self.generate_instance();
        self.battle_entity_instance.init(instance);
    }
}

#[godot_api]
impl BattleModule {
    pub(crate) fn get_battle_module_from_entity(entity: Gd<BoardEntity>) -> Option<Gd<BattleModule>> {
        get_first_child_of_type::<BattleModule>(&entity)
    }

    /** Gets cached entity, returning the same instance in different calls */
    pub(crate) fn get_battle_entity_instance(&self) -> Gd<BattleEntity> {
        self.battle_entity_instance.get_property()
    }
    
    /** Generates new entity, returning the different instances in different calls */
    pub(crate) fn generate_new_battle_entity_instance(&mut self) -> Gd<BattleEntity> {
        self.generate_instance()
    }

    fn generate_instance(&mut self) -> Gd<BattleEntity> {
        let mut new_instance = self.battle_entity_scene.instantiate_as::<BattleEntity>();
        let real_stats = self.stats.bind().get_stats();
        let skills = self.get_skills().unwrap();
        let skill_resources = self.get_skill_resources().unwrap();

        add_stats_to_entity(real_stats.clone(), &mut new_instance);
        add_hp_bar(real_stats.clone(), &mut new_instance);
        add_skills_to_entity(skills, &mut new_instance);
        add_skill_resources_to_entity(skill_resources, &mut new_instance);

        new_instance
    }
}

fn add_stats_to_entity(stats: Gd<RealStats>, entity: &mut Gd<BattleEntity>) {
    entity.bind_mut().set_stats(stats);
}

fn add_hp_bar(stats_to_track: Gd<RealStats>, entity: &mut Gd<BattleEntity>) {
    let mut hp_bar: Gd<HpBar> = HpBar::new_alloc();
    hp_bar.bind_mut().setup(stats_to_track);

    entity.add_child(&hp_bar);
}

fn add_skills_to_entity(skills: Gd<SkillContainerModule>, entity: &mut Gd<BattleEntity>) {
    entity.bind_mut().set_skills(skills);
}

fn add_skill_resources_to_entity(skill_resources: Gd<SkillResourceModule>, entity: &mut Gd<BattleEntity>) {
    entity.bind_mut().set_skill_resource(skill_resources.clone());
    
    let ro_entity = entity as &Gd<BattleEntity>;
    skill_resources.signals().added_to_battle_entity().emit(ro_entity);
}
