use godot::builtin::Vector2i;
use godot::classes::TileMapLayer;
use godot::obj::Gd;

use crate::board::graphics::draw_tile::{DrawTile, FOUR_WAY_DRAW_TILE};

pub(crate) fn verify_tile_set_exists(tile_map_layer: &Gd<TileMapLayer>) {
    if tile_map_layer.get_tile_set().is_none() {
        panic!("No tileset provided for the board!");
    }
}

pub(crate) fn is_tile_empty(board: &Gd<TileMapLayer>, coordinates: Vector2i) -> bool {
    let cell = board.get_cell_tile_data(coordinates);
    return cell.is_none();
}

pub fn add_four_way_draw_tile(board: &mut Gd<TileMapLayer>, coordinates: Vector2i) {
    add_draw_tile(board, coordinates, FOUR_WAY_DRAW_TILE);
}

// https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html#class-tilemaplayer-method-set-cell
fn add_draw_tile(board: &mut Gd<TileMapLayer>, coordinates: Vector2i, tile: DrawTile) {
    board
        .set_cell_ex(coordinates)
        .source_id(tile.get_source_id())
        .atlas_coords(tile.get_atlas_coords())
        .alternative_tile(tile.get_alternative_tile())
        .done();
}

