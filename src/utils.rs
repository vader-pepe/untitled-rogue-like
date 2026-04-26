use raylib::ffi::Vector2;

use crate::state_manager::Direction;

pub fn normalize_coordinate(width: i32, height: i32) -> Vector2 {
    Vector2 {
        x: (width / 2) as f32,
        y: (height / 2) as f32,
    }
}

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
