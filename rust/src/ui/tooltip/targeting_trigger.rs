use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::global_signals::GlobalSignals;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct TargetingTrigger {
    base: Base<Control>,
    #[export]
    entity: OnEditor<Gd<BattleEntity>>,
}

#[godot_api]
impl IControl for TargetingTrigger {
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl TargetingTrigger {
    fn setup(&mut self) {
        self.signals().mouse_entered().connect_self(|_this| {
            godot_print!("Hover - in");
        });

        self.signals().mouse_exited().connect(|| {
            godot_print!("Hover - out");
        });

        self.signals().gui_input().connect_self(
            |this: &mut TargetingTrigger, event: Gd<InputEvent>| {
                let is_pressed = event.is_pressed();
                if !is_pressed {
                    return;
                }

                let mouse_event = event.try_cast::<InputEventMouseButton>();
                if mouse_event.is_err() {
                    return;
                }

                godot_print!("CLICKED");
                let battle_entity = this.get_entity().unwrap();
                GlobalSignals::get_singleton()
                    .signals()
                    .entity_targeted_via_ui()
                    .emit(&battle_entity);
            },
        );
    }
}
