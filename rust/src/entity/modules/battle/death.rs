use godot::classes::*;
use godot::prelude::*;

use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::stats::stats::StatsModule;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct DeathModule {
    entity: OnReady<Gd<BoardEntity>>,
    #[export]
    stats: OnEditor<Gd<StatsModule>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for DeathModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            stats: OnEditor::default(),
            entity: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let parent = self.base_mut().to_godot().clone().upcast::<Node>().get_parent().unwrap();
        self.entity.init(parent.cast::<BoardEntity>()); 

        self.stats.signals().no_hp_left().connect_other(self, Self::on_no_hp_left);
    }
}

impl DeathModule {
    fn on_no_hp_left(&mut self) {
        self.entity.call_deferred("queue_free", &[]);
    }
}
