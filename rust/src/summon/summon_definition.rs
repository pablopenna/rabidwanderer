use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, PartialEq, Debug)]
#[godot(via = GString)]
pub(crate) enum SummonDefinition {
    Spirit,
}
