use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub(crate) struct BaseStats {
    base: Base<Resource>,
    #[export]
    max_hp: u16,
    #[export]
    attack: u16,
    #[export]
    speed: u16,
}