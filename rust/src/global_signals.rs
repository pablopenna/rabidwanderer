use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub(crate) struct GlobalSignals {
    base: Base<Object>,
}

#[godot_api]
impl GlobalSignals {
    #[signal]
    pub(crate) fn player_died();

    pub(crate) fn get_singleton() -> Gd<GlobalSignals> {
        godot::classes::Engine::singleton().get_singleton("GlobalSignals").unwrap().cast::<GlobalSignals>()
    }
}