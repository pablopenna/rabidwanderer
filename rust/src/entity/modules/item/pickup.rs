use godot::classes::*;
use godot::prelude::*;

use crate::board::board::Board;
use crate::consts::groups::get_board_node_from_tree;
use crate::entity::board_entity::BoardEntity;
use crate::entity::modules::item::floor_item::FloorItemModule;
use crate::entity::modules::item::inventory::InventoryModule;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PickupModule {
    base: Base<Node>,
    entity: OnReady<Gd<BoardEntity>>,
    #[export]
    inventory: OnEditor<Gd<InventoryModule>>,
    board: Option<Gd<Board>>,
}

#[godot_api]
impl INode for PickupModule {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            entity: OnReady::manual(),
            inventory: OnEditor::default(),
            board: None,
        }
    }

    fn ready(&mut self) {
        let entity = self.base().get_parent().unwrap().cast::<BoardEntity>();
        self.entity.init(entity);
        
        self.entity.signals()
            .moved_board_tile()
            .connect_other(self, Self::check_for_items_to_pick_up);
    }
}

impl PickupModule {

    pub(crate) fn check_for_items_to_pick_up(&mut self) {
        let coord = self.entity.clone();
        let coord = coord.bind();
        let coord = coord.get_coordinates();

        let tile = self.get_board().bind().get_tile_at(coord).unwrap();
        let entities = tile.bind().get_entities();
        let floor_items: Array<Gd<FloorItemModule>> = entities.iter_shared()
            .map(|e| FloorItemModule::get_floor_item_from_entity(e))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .collect();

        floor_items.iter_shared().for_each(|mut floor_item| {
            if self.inventory.bind().has_room() {
                let item = floor_item.bind_mut().pickup();
                self.inventory.bind_mut().add_item(item)
            }
        });
    }

    fn get_board(&mut self) -> Gd<Board> {
        if self.board.is_none() {
            self.board = Some(get_board_node_from_tree(self.base().clone().upcast::<Node>()));
        }

        self.board.clone().unwrap()
    }
}
