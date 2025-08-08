use godot::prelude::*;

mod test;
pub(crate) mod board; // https://stackoverflow.com/a/63766603
mod player;
mod game_manager;

struct MyExtension; // The name of the struct does not matter

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
