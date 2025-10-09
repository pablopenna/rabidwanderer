use godot::classes::*;
use godot::prelude::*;

use crate::global_signals::GlobalSignals;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct TooltipTrigger {
    base: Base<Control>,
}

#[godot_api]
impl IControl for TooltipTrigger {
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl TooltipTrigger {

    fn setup(&mut self) {
        self.signals().mouse_entered().connect(|| {
            GlobalSignals::get_singleton().signals().show_tooltip().emit("meow");
        });
        
        self.signals().mouse_exited().connect(|| {
            GlobalSignals::get_singleton().signals().hide_tooltip().emit();
        });
    }
}
