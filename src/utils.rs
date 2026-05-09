use raylib::ffi::Rectangle;
use tiled_json_rs::TileSet;

use crate::{
    constants::{TILE_HEIGHT, TILE_WIDTH},
    state_manager::Direction,
};

pub fn get_facing(dx: f32, dy: f32) -> Option<Direction> {
    if dy < 0.0 && dy.abs() >= dx.abs() {
        Some(Direction::North)
    } else if dx > 0.0 && dx >= dy.abs() {
        Some(Direction::East)
    } else if dy > 0.0 && dy >= dx.abs() {
        Some(Direction::South)
    } else if dx < 0.0 && dx.abs() > dy.abs() {
        Some(Direction::West)
    } else {
        None
    }
}

pub fn breakdown_tiles(data: &Vec<TileSet>) -> Vec<Rectangle> {
    let mut tiles_arr: Vec<Rectangle> = vec![];

    for (_, tiles) in data.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;

        // if tiles first_gid is 1, not 0
        // thanks to Tiled, first tile is always empty
        if tiles.first_gid == 1 {
            tiles_arr.push(Rectangle {
                x: -TILE_WIDTH as f32,
                y: -TILE_HEIGHT as f32,
                height: 0.0,
                width: 0.0,
            });
        }
        for _ in 0..tiles.tile_count {
            tiles_arr.push(Rectangle {
                x: x as f32,
                y: y as f32,
                width: tiles.tile_width as f32,
                height: tiles.tile_height as f32,
            });
            x += tiles.tile_width;
            if x >= (tiles.columns * tiles.tile_width) {
                x = 0;
                y += tiles.tile_height;
            }
        }
    }

    tiles_arr
}
