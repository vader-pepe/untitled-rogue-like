use crate::entities::Pos;

#[derive(Debug)]
pub struct Window {
    pub height: i32,
    pub width: i32,
}

impl Window {
    pub fn new(width: i32, height: i32) -> Window {
        Window { height, width }
    }
}

pub fn normalize_coordinate(width: i32, height: i32) -> Pos {
    Pos {
        x: width / 2,
        y: height / 2,
    }
}
