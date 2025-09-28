use godot::classes::*;
use godot::prelude::*;

use crate::battle::ui::skill_button::SkillButton;
use crate::entity::modules::skill::skill_container::SkillContainerModule;
use crate::global_signals::GlobalSignals;
use crate::skill::skill::Skill;

#[derive(GodotClass)]
#[class(init, base=GridContainer)]
pub struct SkillSelectorContainer {
    #[export]
    skill_button_template: OnEditor<Gd<PackedScene>>,
    base: Base<GridContainer>,
}

#[godot_api]
impl IGridContainer for SkillSelectorContainer {
    fn ready(&mut self) {
        self.setup();
    }
}

impl SkillSelectorContainer {
    fn setup(&mut self) {
        GlobalSignals::get_singleton().signals().show_skills_in_battle_ui().connect_other(self, Self::on_show_skills_in_ui);
        GlobalSignals::get_singleton().signals().battle_ui_hid().connect_other(self, Self::remove_all_buttons);
    }

    fn on_show_skills_in_ui(&mut self, skill_container: Gd<SkillContainerModule>) {
        self.remove_all_buttons();

        let skills = skill_container.bind().get_skills();
        skills.iter_shared().for_each(|skill| self.show_skill_in_ui(skill));
    }

    fn show_skill_in_ui(&mut self, skill: Gd<Skill>) {
        let button = self.instance_button_template(skill);
        Self::setup_button(&button);

        
        self.add_button(&button);
    }

    fn instance_button_template(&mut self, skill: Gd<Skill>) -> Gd<SkillButton> {
        let template = self.get_skill_button_template().unwrap();
        let mut button = template.instantiate_as::<SkillButton>();
        button.bind_mut().set_linked_skill(Some(skill));

        button
    }

    fn add_button(&mut self, button: &Gd<SkillButton>) {
        self.base_mut().add_child(button);
    }

    fn setup_button(button: &Gd<SkillButton>) {
        button.signals().skill_button_pressed().connect(|skill| 
            GlobalSignals::get_singleton().signals().skill_chosen_in_battle_ui().emit(&skill)
        );
    }

    fn remove_all_buttons(&mut self) {
        let children = self.base().get_children();
        children.iter_shared().for_each(|node| self.base_mut().remove_child(&node));
    }
}

