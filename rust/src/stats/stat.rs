use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub enum Stat {
    MaxHp,
    #[default]
    Attack,
    Speed,
}