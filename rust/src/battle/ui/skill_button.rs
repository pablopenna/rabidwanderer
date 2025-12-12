use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::skill::Skill;

#[derive(GodotClass)]
#[class(base=BoxContainer)]
pub struct SkillButton {
    #[var]
    linked_skill: Option<Gd<Skill>>, // set on instantiation
    #[var]
    skill_resource: Option<Gd<SkillResourceModule>>, // set on instantiation
    #[export]
    button: OnEditor<Gd<Button>>,
    #[export]
    resource_label: OnEditor<Gd<Label>>,
    base: Base<BoxContainer>,
}

#[godot_api]
impl IBoxContainer for SkillButton {
    fn init(base: Base<BoxContainer>) -> Self {
        Self {
            linked_skill: None,
            skill_resource: None,
            button: OnEditor::default(),
            resource_label: OnEditor::default(),
            base,
        }
    }
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl SkillButton {
    #[signal]
    pub(crate) fn skill_button_pressed(skill: Gd<Skill>, skill_resource: Gd<SkillResourceModule>);

    fn setup(&mut self) {
        let button = self.get_button().unwrap();

        button
            .signals()
            .pressed()
            .connect_other(self, Self::on_button_pressed);

        self.setup_icon();
        self.update_resource_label();
    }

    fn setup_icon(&mut self) {
        let skill = self.linked_skill.clone().unwrap();
        let icon = skill.bind().get_icon();
        if icon.is_none() {
            return;
        }

        let mut button = self.get_button().unwrap();
        button.set_button_icon(&icon.unwrap());
    }

    fn update_resource_label(&mut self) {
        let skill = self.linked_skill.clone().unwrap();
        let skill_definition = skill.bind().get_definition();

        let resource = self.get_skill_resource().unwrap();
        let resource_implementation = resource.bind().get_implementation();
        let available_resource = resource_implementation
            .dyn_bind()
            .get_available_resource_for(skill_definition)
            .to_string();

        let mut label = self.get_resource_label().unwrap();
        label.set_text(&available_resource);
    }

    fn on_button_pressed(&mut self) {
        let skill = self.linked_skill.clone().unwrap();
        let skill_resource = self.skill_resource.clone().unwrap();
        let can_cast = skill_resource
            .bind()
            .has_resources_to_cast(skill.bind().get_definition());

        if can_cast {
            self.signals()
                .skill_button_pressed()
                .emit(&skill, &skill_resource);
        } else {
            godot_print!("Cannot choose that!");
        }
    }
}
