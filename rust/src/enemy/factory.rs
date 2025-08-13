use godot::classes::*;
use godot::prelude::*;

use crate::consts::groups::ENEMY_FACTORY_GROUP;
use crate::enemy::enemy_definition::EnemyDefinition;
use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct EnemyFactory {
    #[export]
    wolf_enemy_scene: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for EnemyFactory {
    fn init(base: Base<Node>) -> Self {
        Self {
            wolf_enemy_scene: OnEditor::default(),
            base
        }
    }

    fn ready(&mut self) {
        let mut node = self.base_mut().to_godot().upcast::<Node>();
        node.add_to_group(ENEMY_FACTORY_GROUP);
    }
}

impl EnemyFactory {
    pub(crate) fn instance_random_enemy(&mut self) -> Gd<BoardEntity> {
        let random_def = EnemyDefinition::random();
        self.instance_enemy(&random_def)
    }

    fn instance_enemy(&self, def: &EnemyDefinition) -> Gd<BoardEntity> {
        let scene = self.get_scene_for_enemy(def);
        let enemy = scene.instantiate_as::<BoardEntity>();
        enemy
    }

    fn get_scene_for_enemy(&self, def: &EnemyDefinition) -> Gd<PackedScene> {
        match def {
            EnemyDefinition::Wolf => self.wolf_enemy_scene.get_property().unwrap(),
        }
    }    
}
