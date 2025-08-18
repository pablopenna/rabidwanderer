use godot::classes::*;
use godot::prelude::*;

use godot::obj::Gd;

use crate::board::coordinate::BoardCoordinate;
use crate::entity::board_entity::BoardEntity;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct DataTile {
    coordinates: BoardCoordinate,
    can_be_traversed: bool,
    base: Base<Node>
    // Read ADR-01 and ADR-02 for design decissions on why these attributes are done this way.
}

#[godot_api]
impl INode for DataTile {
    fn init(base: Base<Node>) -> Self {
        Self {
            coordinates: BoardCoordinate::from_vector2i(
                Vector2i {x: -1, y: -1}
            ),
            can_be_traversed: false,
            base,
        }
    }

    // Just for debug purposes
    fn ready(&mut self) {
        let x = self.get_coordinates().get_x();
        let y = self.get_coordinates().get_y();
        let name = format!("{x}x{y}");
        self.base_mut().set_name(&name);
    }
}

impl DataTile {
    pub(crate) fn new(coord: BoardCoordinate) -> Gd<Self> {
        let mut new_tile = Self::new_alloc();
        let mut bind = new_tile.bind_mut();
        bind.set_coordinates(coord);
        drop(bind);

        new_tile
    }

    pub(crate) fn get_coordinates(&self) -> &BoardCoordinate {
        &self.coordinates
    }
    
    fn set_coordinates(&mut self, coord: BoardCoordinate) {
        self.coordinates = coord;
    }
    
    pub(crate) fn make_traversable(&mut self) {
        self.can_be_traversed = true;
    }

    pub(crate) fn is_traversable(&self) -> bool {
        self.can_be_traversed
    }

    pub(crate) fn add_entity_to_tile(& mut self, entity: &Gd<BoardEntity>) {
        self.base_mut().add_child(entity);
    }

    pub(crate) fn contains_entity(& mut self, entity: Gd<BoardEntity>) -> bool {
        let children = self.base_mut().get_children();
        children.contains(&entity.upcast::<Node>())
    }

    pub(crate) fn get_entities(&self) -> Array<Gd<BoardEntity>> {
        self.base().get_children().iter_shared().map(|child| child.cast::<BoardEntity>()).collect()
    }

    pub(crate) fn is_data_tile_traversable(tile: Option<Gd<DataTile>>) -> bool {
        tile.is_some() && tile.unwrap().bind().is_traversable()
    }

    pub(crate) fn move_entity_to(entity_to_move: &mut Gd<BoardEntity>, tile: &mut Gd<DataTile>) {
        let tile_node = tile.clone().upcast::<Node>();
        // If call is not deferred it will throw an error saying that player already has a parent (though it seems it works properly either way)
        // entity_to_move.call_deferred("reparent", &[tile_node.to_variant()]);
        entity_to_move.reparent(&tile_node);
    }

    pub(crate) fn move_entity_to_deferred(entity_to_move: &mut Gd<BoardEntity>, tile: &mut Gd<DataTile>) {
        let tile_node = tile.clone().upcast::<Node>();
        // If call is not deferred it will throw an error saying that player already has a parent (though it seems it works properly either way)
        entity_to_move.call_deferred("reparent", &[tile_node.to_variant()]);
    }
}

