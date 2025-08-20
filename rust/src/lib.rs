use godot::prelude::*;
use godot::classes::Engine;
use global_signals::GlobalSignals;

mod consts;
mod game_manager;
mod board;
mod entity;
mod camera;
mod item;
mod enemy;
mod battle;
mod stats;
mod global_signals;
mod utils;

struct MyExtension; // The name of the struct does not matter

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    // https://godot-rust.github.io/book/recipes/engine-singleton.html#registering-a-singleton
    
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // The `&str` identifies your singleton and can be used later to access it.
            Engine::singleton().register_singleton(
                &GlobalSignals::class_name().to_string_name(),
                &GlobalSignals::new_alloc(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = &GlobalSignals::class_name().to_string_name();

            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                godot_error!("Failed to get singleton to free it during deinit");
            }
        }
    }
}
