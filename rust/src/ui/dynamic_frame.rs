use godot::classes::*;
use godot::prelude::*;

// It is recommended to use the DynamicFrame Scene in the Godot project
// instead of this struct directly as the texture needs to be set.
#[derive(GodotClass)]
#[class(init, base=NinePatchRect)]
pub struct DynamicFrame {
    base: Base<NinePatchRect>,
    #[export]
    target: Option<Gd<Sprite2D>>,
    #[export]
    margin: f32,
}

// #[godot_api]
// impl INinePatchRect for DynamicFrame {
//     fn ready(&mut self) {
//         self.adjust_to_target();
//     }
// }

#[godot_api]
impl DynamicFrame {
    pub(crate) fn hide(&mut self) {
        self.base_mut().set_visible(false);
    }

    pub(crate) fn adjust_to(&mut self, target: &Gd<Sprite2D>) {
        self.set_target(Some((*target).clone()));
        self.adjust_to_target();
    }

    pub(crate) fn adjust_to_target(&mut self) {
        self.base_mut().set_visible(true);
        self.adjust_size_to_target();
        self.adjust_position_to_target();
    }

    fn adjust_position_to_target(&mut self) {
        let target = self.get_target().unwrap();
        self.base_mut()
            .set_global_position(target.get_global_position());

        // Sprite position is the center of the sprite but the NinePatchRect position it is its upper left corner.
        // Here we asume that the size has already been adjusted
        let size = self.base().get_rect().size;
        let horizontal_adjustment_offset = size.x * 0.5;
        let vertical_adjustment_offset = size.y * 0.5;
        let position = self.base().get_position();

        self.base_mut().set_position(Vector2 {
            x: position.x - horizontal_adjustment_offset,
            y: position.y - vertical_adjustment_offset,
        });
    }

    fn adjust_size_to_target(&mut self) {
        let target = self.get_target().unwrap();
        let size = target.get_rect().size
            + Vector2 {
                x: self.margin * 2f32,
                y: self.margin * 2f32,
            };
        self.base_mut().set_size(size);
    }
}
