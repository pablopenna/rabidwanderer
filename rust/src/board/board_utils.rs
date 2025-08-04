
use godot::classes::TileMapLayer;
use godot::obj::Gd;

pub fn verify_tile_set_exists(tile_map_layer: Gd<TileMapLayer>){
    if tile_map_layer.get_tile_set().is_none() {
        panic!("No tileset provided for the board!");
    }
}