use godot::classes::*;
use godot::prelude::*;

use crate::stats::real::RealStats;

const HEIGHT_PX: f32 = 10.;
const WIDTH_PX: f32 = 40.;
const X_POSITION_OFFSET: f32 = 0.;
const Y_POSITION_OFFSET: f32 = -20.;

// This class should be a module/node under the battle-entity (i.e. the direct parent should be the battle entity being tracked).
#[derive(GodotClass)]
#[class(base=ProgressBar)]
pub struct HpBar {
    tracked_stats: Option<Gd<RealStats>>,
    hp_label: Gd<Label>,
    base: Base<ProgressBar>,
}

#[godot_api]
impl IProgressBar for HpBar {
    fn init(base: Base<ProgressBar>) -> Self {
        Self {
            tracked_stats: None,
            hp_label: Label::new_alloc(),
            base
        }
    }

    fn ready(&mut self) {
        self.update_label_text();
        self.update_label_position();
    }
}

#[godot_api]
impl HpBar {
    pub(crate) fn setup(&mut self, stats_to_track: Gd<RealStats>) {
        self.setup_hp_values(stats_to_track);
        self.setup_progress_bar_properties();
        self.setup_label();
        godot_print!("setting up hp bar!");
    }

    fn setup_hp_values(&mut self, stats_to_track: Gd<RealStats>) {
        self.tracked_stats = Some(stats_to_track);
        // default 'changed' signal from Resource cannot be used for custom properties of custom Resources - https://docs.godotengine.org/en/stable/classes/class_resource.html#class-resource-signal-changed
        self.tracked_stats.as_ref().unwrap().signals().hp_changed().connect_other(self, Self::on_hp_changed);

        let max_hp = self.tracked_stats.as_ref().unwrap().bind().get_max_hp();
        self.base_mut().set_max(max_hp as f64); 
        let current_hp = self.tracked_stats.as_ref().unwrap().bind().get_current_hp();
        self.base_mut().set_value(current_hp as f64); 
    }

    fn setup_progress_bar_properties(&mut self) {
        self.base_mut().set_show_percentage(false);
        self.base_mut().set_size(Vector2 {x: WIDTH_PX, y: HEIGHT_PX});
        self.base_mut().set_position(Vector2 {x: X_POSITION_OFFSET, y: Y_POSITION_OFFSET});
    }

    fn setup_label(&mut self) {
        let label_ref = &self.hp_label.clone();
        self.base_mut().add_child(label_ref);

        self.update_label_text();
        self.update_label_position();
    }

    // place on top of the progress bar
    fn update_label_position(&mut self) {
        // let progress_bar_size = self.base().get_size();
        let label_size = self.hp_label.get_size();
        godot_print!("{}", label_size);

        self.hp_label.set_position(Vector2 {x:0.0, y:-label_size.y}); // position should be relative to progress bar as this is its child
    }

    fn update_label_text(&mut self) {
        let stats = self.tracked_stats.clone().unwrap();
        let current_hp = stats.bind().get_current_hp();
        let max_hp = stats.bind().get_max_hp();
        let label_text = format!("{}/{}", current_hp, max_hp);
        
        self.hp_label.set_text(&label_text);
    }

    fn on_hp_changed(&mut self, new_hp: u16) {
        self.base_mut().set_value(new_hp as f64);
        self.update_label_text();
        self.update_label_position();
    }
}


