use godot::classes::*;
use godot::prelude::*;

use crate::entity::modules::skill::skill_resource::SkillResourceModule;
use crate::skill::skill::Skill;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(base=BoxContainer)]
pub struct SkillButton {
    #[var]
    linked_skill: Option<Gd<Skill>>, // set on instantiation
    #[var]
    skill_resource: Option<Gd<SkillResourceModule>>, // set on instantiation
    #[export]
    button: OnEditor<Gd<Button>>,
    base: Base<BoxContainer>,
}

#[godot_api]
impl IBoxContainer for SkillButton {
    fn init(base: Base<BoxContainer>) -> Self {
        Self {
            linked_skill: None,
            skill_resource: None,
            button: OnEditor::default(),
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

    fn on_button_pressed(&mut self) {
        let skill = self.linked_skill.clone().unwrap();
        let skill_resource = self.skill_resource.clone().unwrap();
        let can_cast = skill_resource
            .bind()
            .has_resources_to_cast(SkillDefinition::from_gstring(skill.bind().get_name()));

        if can_cast {
            self.signals()
                .skill_button_pressed()
                .emit(&skill, &skill_resource);
        } else {
            godot_print!("Cannot choose that!");
        }
    }
}
