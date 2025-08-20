use godot::classes::*;
use godot::prelude::*;

use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::item::floor_item::FloorItemModule;
use crate::item::item_definition::ItemDefinition;
use crate::utils::get_first_child_of_type::get_first_child_of_type;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FloorItemFactory {
    #[export]
    floor_item_board_entity_scene: OnEditor<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for FloorItemFactory {
    fn init(base: Base<Node>) -> Self {
        Self {
            floor_item_board_entity_scene: OnEditor::default(),
            base
        }
    }
}

impl FloorItemFactory {
    pub(crate) fn create_floor_item(&self, def: ItemDefinition) -> Gd<BoardEntity> {
        let board_entity = self.floor_item_board_entity_scene.instantiate_as::<BoardEntity>();
        let board_entity = board_entity.upcast::<Node>();
        
        let floor_item_module = get_first_child_of_type::<FloorItemModule>(&board_entity);
        let mut floor_item_module = floor_item_module.unwrap();
        let mut floor_item_module = floor_item_module.bind_mut();
       
        floor_item_module.set_item(def);
        drop(floor_item_module);
        
        board_entity.cast::<BoardEntity>()
    }

    pub(crate) fn create_random_floor_item(&mut self) -> Gd<BoardEntity> {
        let random_def = ItemDefinition::random();
        self.create_floor_item(random_def)
    }
}
