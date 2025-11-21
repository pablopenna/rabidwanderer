use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::global_signals::GlobalSignals;
use crate::ui::dynamic_frame::DynamicFrame;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TargetingTrigger {
    base: Base<Control>,
    #[export]
    entity: OnEditor<Gd<BattleEntity>>,
    #[export]
    frame_scene: OnEditor<Gd<PackedScene>>, // Need a scene so that the texture is already set
    frame: OnReady<Gd<DynamicFrame>>,
}

#[godot_api]
impl IControl for TargetingTrigger {
    fn init(base: Base<Control>) -> Self {
        TargetingTrigger {
            base,
            entity: OnEditor::default(),
            frame_scene: OnEditor::default(),
            frame: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl TargetingTrigger {
    fn setup(&mut self) {
        self.instance_frame();

        self.setup_mouse_entered();
        self.setup_mouse_exited();
        self.setup_mouse_clicked();
    }

    fn instance_frame(&mut self) {
        let scene = self.get_frame_scene().unwrap();
        let mut instance = scene.instantiate_as::<DynamicFrame>();

        instance.bind_mut().hide();
        self.base_mut().add_child(&instance);
        self.frame.init(instance);
    }

    fn setup_mouse_entered(&mut self) {
        let mut frame = self.frame.clone();
        let entity_sprite = self.get_entity().unwrap().bind().get_sprite().unwrap();

        self.signals().mouse_entered().connect(move || {
            godot_print!("Hover - in");
            frame.bind_mut().adjust_to(entity_sprite.clone());
        });
    }

    fn setup_mouse_exited(&mut self) {
        let mut frame = self.frame.clone();

        self.signals().mouse_exited().connect(move || {
            godot_print!("Hover - out");
            frame.bind_mut().hide();
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
                    .entity_targeted_via_ui()
                    .emit(&battle_entity);
            },
        );
    }
}
