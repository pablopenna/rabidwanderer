use godot::classes::*;
use godot::prelude::*;

use crate::stats::real::RealStats;

const HEIGHT_PX: f32 = 4.;
const WIDTH_PX: f32 = 20.;
const X_POSITION_OFFSET: f32 = 0.;
const Y_POSITION_OFFSET: f32 = -20.;

// This class should be a module/node under the battle-entity (i.e. the direct parent should be the battle entity being tracked).
#[derive(GodotClass)]
#[class(init, base=ProgressBar)]
pub struct HpBar {
    tracked_stats: Option<Gd<RealStats>>,
    base: Base<ProgressBar>,
}

#[godot_api]
impl HpBar {
    pub(crate) fn setup(&mut self, stats_to_track: Gd<RealStats>) {
        self.tracked_stats = Some(stats_to_track);
        // default 'changed' signal from Resource cannot be used for custom properties of custom Resources - https://docs.godotengine.org/en/stable/classes/class_resource.html#class-resource-signal-changed
        self.tracked_stats.as_ref().unwrap().signals().hp_changed().connect_other(self, Self::on_hp_changed);

        let max_hp = self.tracked_stats.as_ref().unwrap().bind().get_max_hp();
        self.base_mut().set_max(max_hp as f64); 
        let current_hp = self.tracked_stats.as_ref().unwrap().bind().get_current_hp();
        self.base_mut().set_value(current_hp as f64); 

        self.base_mut().set_show_percentage(false);
        self.base_mut().set_size(Vector2 {x: WIDTH_PX, y: HEIGHT_PX});
        self.base_mut().set_position(Vector2 {x: X_POSITION_OFFSET, y: Y_POSITION_OFFSET});
    }

    fn on_hp_changed(&mut self, new_hp: u16) {
        self.base_mut().set_value(new_hp as f64);
    }
}


