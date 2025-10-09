use godot::classes::*;
use godot::prelude::*;

use crate::battle::ui::skill_button::SkillButton;
use crate::global_signals::GlobalSignals;
use crate::skill::skill_definition::SkillDefinition;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct SkillTooltipTrigger {
    base: Base<Control>,
    #[export]
    skill_button: OnEditor<Gd<SkillButton>>,
}

#[godot_api]
impl IControl for SkillTooltipTrigger {
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl SkillTooltipTrigger {

    fn setup(&mut self) {
        self.signals().mouse_entered().connect_self(| this | {
            GlobalSignals::get_singleton().signals().show_tooltip().emit(&this.get_skill_text());
        });
        
        self.signals().mouse_exited().connect(|| {
            GlobalSignals::get_singleton().signals().hide_tooltip().emit();
        });
    }

    fn get_skill_text(&mut self) -> GString {
        let skill_button = self.get_skill_button().unwrap();
        let skill = skill_button.bind().get_linked_skill().unwrap();
        let skill_name = skill.bind().get_name();
        let description = SkillDefinition::from_gstring(skill_name).get_description();
        
        description.into()
    }
}
