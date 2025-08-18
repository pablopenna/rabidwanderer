use godot::classes::*;
use godot::prelude::*;

use crate::board::coordinate::BoardCoordinate;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BoardEntity {
    coordinates: BoardCoordinate,
    base: Base<Node2D>,
}

#[godot_api]
impl BoardEntity {

    #[signal]
    pub(crate) fn on_interact(self_reference: Gd<BoardEntity>, interacted_with: Gd<BoardEntity>);

    #[signal]
    pub(crate) fn moved_board_tile();

    #[signal]
    pub(crate) fn added_to_board();

    pub(crate) fn interact_with(&mut self, entity: &Gd<BoardEntity>) {
        let gd_ref = self.to_gd();
        self.signals().on_interact().emit(&gd_ref, entity);
    }

    pub(crate) fn get_coordinates(&self) -> &BoardCoordinate {
        &self.coordinates
    }
    
    pub(crate) fn set_coordinates(&mut self, coord: BoardCoordinate) {
        self.coordinates = coord;
    }

    pub(crate) fn set_world_position(&mut self, position: Vector2) {
        // let global_position = self.base().to_godot().to_global(position);
        self.base_mut().to_godot().set_position(position);
    }

    // https://doc.rust-lang.org/book/ch10-01-syntax.html
    pub(crate) fn get_first_child_of_type<T>(parent: &Node) -> Option<Gd<T>> 
    where 
        T: GodotClass + Inherits<godot::prelude::Node> 
    {
        for i in 0..parent.get_child_count() {
            let child = parent.get_child(i);
            if child.is_none() {
                continue;
            }
            let child = child.unwrap();
            let child = child.try_cast::<T>();
            if child.is_ok() {
                return child.ok();
            }
        }
        None
    }
}


