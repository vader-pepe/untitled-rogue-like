use raylib::ffi::Vector2;

pub fn normalize_coordinate(width: i32, height: i32) -> Vector2 {
    Vector2 {
        x: (width / 2) as f32,
        y: (height / 2) as f32,
    }
}
