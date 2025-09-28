use godot::classes::*;
use godot::prelude::*;

use crate::skill::skill::Skill;

#[derive(GodotClass)]
#[class(base=Button)]
pub struct SkillButton {
    #[export]
    linked_skill: Option<Gd<Skill>>, // set on instantiation
    base: Base<Button>,
}

#[godot_api]
impl IButton for SkillButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            linked_skill: None,
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
    pub(crate) fn skill_button_pressed(skill: Gd<Skill>);

    fn setup(&mut self) {
        self.signals().pressed().connect_self(Self::on_button_pressed);   
        
        self.setup_icon();
    }

    fn setup_icon(&mut self) {
        let skill = self.linked_skill.clone().unwrap();
        let icon = skill.bind().get_icon();
        if icon.is_none() {
            return;
        }
        self.base_mut().set_button_icon(&icon.unwrap());
    }

    fn on_button_pressed(&mut self) {
        let skill = self.linked_skill.clone().unwrap();
        self.signals().skill_button_pressed().emit(&skill);
    }
}

