use godot::prelude::*;

mod player;
mod board; // https://stackoverflow.com/a/63766603

struct MyExtension; // The name of the struct does not matter

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
