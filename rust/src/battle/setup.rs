use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub(crate) struct BattleSetup {
    base: Base<Node>,
    #[export]
    player_position_reference: OnEditor<Gd<Node2D>>,
    #[export]
    enemy_position_reference: OnEditor<Gd<Node2D>>,
    #[export]
    battle_entities_container: OnEditor<Gd<Node>>,

}

#[godot_api]
impl BattleSetup {
    
}


