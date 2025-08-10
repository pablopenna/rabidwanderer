use godot::prelude::*;

pub(crate) mod consts;
mod game_manager;
pub(crate) mod board; // https://stackoverflow.com/a/63766603
mod entity;
mod camera;

struct MyExtension; // The name of the struct does not matter

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
