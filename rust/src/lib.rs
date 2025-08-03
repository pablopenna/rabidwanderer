use godot::prelude::*;

mod player;
mod board;

struct MyExtension; // The name of the struct does not matter

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
