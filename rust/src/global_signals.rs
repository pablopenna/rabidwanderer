use godot::prelude::*;

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

    pub(crate) fn get_singleton() -> Gd<GlobalSignals> {
        godot::classes::Engine::singleton().get_singleton("GlobalSignals").unwrap().cast::<GlobalSignals>()
    }
}