use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::global_signals::GlobalSignals;

// Detects if battle entity is hovered or clicked and emits signal
#[derive(GodotClass)]
#[class(base=Control)]
pub struct TargetingTrigger {
    base: Base<Control>,
    #[export]
    entity: OnEditor<Gd<BattleEntity>>,
}

#[godot_api]
impl IControl for TargetingTrigger {
    fn init(base: Base<Control>) -> Self {
        TargetingTrigger {
            base,
            entity: OnEditor::default(),
        }
    }

    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl TargetingTrigger {
    fn setup(&mut self) {
        self.setup_mouse_entered();
        self.setup_mouse_exited();
        self.setup_mouse_clicked();
    }

    fn setup_mouse_entered(&mut self) {
        let battle_entity = self.get_entity().unwrap();

        self.signals().mouse_entered().connect(move || {
            GlobalSignals::get_singleton()
                .signals()
                .battle_entity_hovered_in_via_ui()
                .emit(&battle_entity);
        });
    }

    fn setup_mouse_exited(&mut self) {
        let battle_entity = self.get_entity().unwrap();

        self.signals().mouse_exited().connect(move || {
            GlobalSignals::get_singleton()
                .signals()
                .battle_entity_hovered_out_via_ui()
                .emit(&battle_entity);
        });
    }

    fn setup_mouse_clicked(&mut self) {
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

                let battle_entity = this.get_entity().unwrap();
                GlobalSignals::get_singleton()
                    .signals()
                    .battle_entity_clicked_via_ui()
                    .emit(&battle_entity);
            },
        );
    }
}
