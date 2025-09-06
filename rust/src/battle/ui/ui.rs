use godot::classes::node::ProcessMode;
use godot::classes::*;
use godot::prelude::*;

use crate::global_signals::GlobalSignals;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BattleUi {
    #[export]
    subviewport: OnEditor<Gd<SubViewport>>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for BattleUi {
    fn ready(&mut self) {
        GlobalSignals::get_singleton().signals().battle_set_up().connect_other(self, Self::show);
        GlobalSignals::get_singleton().signals().battle_finished().connect_other(self, Self::hide);
    }
}

#[godot_api]
impl BattleUi {

    pub(crate) fn _pause(&mut self) {
        self.subviewport.set_process_mode(ProcessMode::DISABLED);
    }

    pub(crate) fn _unpause(&mut self) {
        self.subviewport.set_process_mode(ProcessMode::INHERIT);
    }

    pub(crate) fn show(&mut self) {
        self.base_mut().set_visible(true);
        GlobalSignals::get_singleton().signals().battle_ui_shown().emit();
    }

    pub(crate) fn hide(&mut self) {
        self.base_mut().set_visible(false);
        GlobalSignals::get_singleton().signals().battle_ui_hid().emit();
    }
}


