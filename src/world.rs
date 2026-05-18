use raylib::prelude::*;

use crate::{
    constants::{TILE_HEIGHT, TILE_WIDTH},
    state_manager::Game,
};

pub fn draw_world(draw_handle: &mut RaylibDrawHandle, game_instance: &mut Game) {
    draw_handle.clear_background(Color::WHITE);

    // TODO: this causes map overlap
    for map in game_instance.map.iter().rev() {
        for map_layer in &map.layers {
            match &map_layer.layer_type {
                tiled_json_rs::LayerType::ImageLayer(_image) => {
                    todo!()
                }
                tiled_json_rs::LayerType::Group { layers: _ } => {
                    todo!()
                }
                tiled_json_rs::LayerType::ObjectGroup(_obj) => {
                    todo!()
                }
                tiled_json_rs::LayerType::TileLayer(tile_layer) => {
                    for (i, gid) in tile_layer.data.iter().enumerate() {
                        if *gid == 0 {
                            continue;
                        }
                        let tile_x = (i as u32 % tile_layer.width) * TILE_WIDTH as u32;
                        let tile_y = (i as u32 / tile_layer.width) * TILE_HEIGHT as u32;

                        if let Some(&rect) = game_instance.tiles.get(*gid as usize) {
                            for (idx, tileset) in map.tile_sets.iter().enumerate() {
                                if *gid >= tileset.first_gid
                                    && *gid < tileset.first_gid + tileset.tile_count
                                {
                                    if let Some(texture) = game_instance.map_textures.get(idx) {
                                        draw_handle.draw_texture_rec(
                                            texture,
                                            rect,
                                            Vector2::new(tile_x as f32, tile_y as f32),
                                            Color::WHITE,
                                        );
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
