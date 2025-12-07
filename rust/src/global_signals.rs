use godot::prelude::*;

use crate::{battle::entity::entity::BattleEntity, entity::modules::skill::{skill_container::SkillContainerModule, skill_resource::SkillResourceModule}, item::item::Item, skill::skill::Skill};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub(crate) struct GlobalSignals {
    base: Base<Object>,
}

#[godot_api]
impl GlobalSignals {

    // General
    #[signal]
    pub(crate) fn game_over();

    // Board

    // Battle
    #[signal]
    pub(crate) fn battle_set_up();
    #[signal]
    pub(crate) fn battle_started();
    #[signal]
    pub(crate) fn battle_finished();

    // UI
    #[signal]
    pub(crate) fn battle_ui_shown();
    #[signal]
    pub(crate) fn battle_ui_hid();

    // Skills
    #[signal]
    pub(crate) fn show_skills_in_battle_ui(skills: Gd<SkillContainerModule>, skill_resource: Gd<SkillResourceModule>);
    #[signal]
    pub(crate) fn skill_chosen_in_battle_ui(skill: Gd<Skill>, skill_resource: Gd<SkillResourceModule>);

    // Inventory
    #[signal]
    pub(crate) fn inventory_ui_item_added(item: Gd<Item>);
    #[signal]
    pub(crate) fn inventory_ui_item_clicked(item: Gd<Item>);

    // Tooltip
    #[signal]
    pub(crate) fn show_tooltip(content: GString);
    #[signal]
    pub(crate) fn hide_tooltip();
    
    // Targeting
    #[signal]
    pub(crate) fn battle_entity_clicked_via_ui(battle_entity: Gd<BattleEntity>);
    #[signal]
    pub(crate) fn battle_entity_hovered_in_via_ui(battle_entity: Gd<BattleEntity>);
    #[signal]
    pub(crate) fn battle_entity_hovered_out_via_ui(battle_entity: Gd<BattleEntity>);
    
    #[signal]
    pub(crate) fn choose_target_via_ui(candidates: Array<Gd<BattleEntity>>);
    #[signal]
    pub(crate) fn targets_chosen_via_ui(selection: Array<Gd<BattleEntity>>);
    

    pub(crate) fn get_singleton() -> Gd<GlobalSignals> {
        godot::classes::Engine::singleton().get_singleton("GlobalSignals").unwrap().cast::<GlobalSignals>()
    }
}