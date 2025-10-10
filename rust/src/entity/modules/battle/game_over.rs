use godot::classes::*;
use godot::prelude::*;

use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::stats::stats::StatsModule;
use crate::global_signals::GlobalSignals;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameOverModule {
    entity: OnReady<Gd<BoardEntity>>,
    #[export]
    stats: OnEditor<Gd<StatsModule>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for GameOverModule {
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

impl GameOverModule {
    fn on_no_hp_left(&mut self) {
        GlobalSignals::get_singleton().signals().game_over().emit();
    }
}
