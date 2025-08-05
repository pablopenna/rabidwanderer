use godot::prelude::*;

mod test;
mod board; // https://stackoverflow.com/a/63766603
mod player;

struct MyExtension; // The name of the struct does not matter

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
