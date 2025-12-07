use godot::classes::*;
use godot::prelude::*;

use crate::battle::entity::entity::BattleEntity;
use crate::global_signals::GlobalSignals;
use crate::ui::dynamic_frame::DynamicFrame;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct UiTargetingHelper {
    base: Base<Node>,
    candidates: Array<Gd<BattleEntity>>,
    is_active: bool,
    
    #[export]
    frame_scene: OnEditor<Gd<PackedScene>>, // Need a scene so that the texture is already set
    frame: OnReady<Gd<DynamicFrame>>,
}

#[godot_api]
impl INode for UiTargetingHelper {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            candidates: Array::default(),
            is_active: false,
            frame_scene: OnEditor::default(),
            frame: OnReady::manual(),
        }
    }
    
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl UiTargetingHelper {
    fn setup(&mut self) {
        self.instance_frame();
        self.setup_signals();
    }
    
    fn instance_frame(&mut self) {
        let scene = self.get_frame_scene().unwrap();
        let mut instance = scene.instantiate_as::<DynamicFrame>();

        instance.bind_mut().hide();
        self.base_mut().add_child(&instance);
        self.frame.init(instance);
    }
    
    fn setup_signals(&mut self) {
        GlobalSignals::get_singleton().signals().choose_target_via_ui().connect_other(self, Self::on_choose_target);
        
        GlobalSignals::get_singleton().signals().battle_entity_hovered_in_via_ui().connect_other(self, Self::on_entity_hovered_in);
        GlobalSignals::get_singleton().signals().battle_entity_hovered_out_via_ui().connect_other(self, Self::on_entity_hovered_out);
        GlobalSignals::get_singleton().signals().battle_entity_clicked_via_ui().connect_other(self, Self::on_entity_clicked);
    }
    
    fn on_choose_target(&mut self, candidates: Array<Gd<BattleEntity>>) {
        self.is_active = true;
        self.candidates.clone_from(&candidates);
    }
    
    fn on_entity_hovered_in(&mut self, entity: Gd<BattleEntity>) {
        if !self.is_valid_target(&entity) {
            godot_print!("Hovered-in entity not a candidate");
            return;
        }
        
        let entity_sprite = entity.bind().get_sprite().unwrap();
        self.frame.bind_mut().adjust_to(&entity_sprite);
    }
    
    fn on_entity_hovered_out(&mut self, entity: Gd<BattleEntity>) {
        if !self.is_valid_target(&entity) {
            godot_print!("Hovered-out entity not a candidate");
            return;
        }
        
        self.frame.bind_mut().hide();
    }
    
    fn on_entity_clicked(&mut self, entity: Gd<BattleEntity>) {
        if !self.is_valid_target(&entity) {
            godot_print!("Hovered-out entity not a candidate");
            return;
        }
        
        self.is_active = false;
        self.frame.bind_mut().hide();
        
        let targets = array!(&entity);
        GlobalSignals::get_singleton().signals().targets_chosen_via_ui().emit(&targets);
    }
    
    fn is_valid_target(&self, entity: &Gd<BattleEntity>) -> bool {
        self.is_active && self.candidates.contains(entity)
    }
}
