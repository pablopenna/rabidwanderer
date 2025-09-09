use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub enum CustomModifier {
    #[default]
    ONE, // outputs 1
}