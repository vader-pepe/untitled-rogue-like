use crate::utils::normalize_coordinate;

#[derive(Debug)]
pub struct Player {
    pub position: Pos,
    pub state: PlayerState,
    pub facing: PlayerFacing,
    pub attack: i32,
    pub attack_range: i32,
}

#[derive(Debug)]
pub enum PlayerFacing {
    South,
    East,
    North,
    West,
}

#[derive(Debug)]
pub enum PlayerState {
    Idle,
    Walk,
    Attack,
}

#[derive(Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new(w_width: i32, w_height: i32) -> Player {
        Player {
            position: normalize_coordinate(w_width, w_height),
            state: PlayerState::Idle,
            attack: 1,
            attack_range: 1,
            facing: PlayerFacing::East,
        }
    }
}
