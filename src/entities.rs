use raylib::{ffi::Vector2, prelude::RaylibDrawHandle};

use crate::utils::normalize_coordinate;

#[derive(Debug)]
pub struct Player {
    pub position: Vector2,
    pub previous_position: Vector2,
    pub state: PlayerState,
    pub facing: PlayerFacing,
    pub attack: i32,
    pub attack_range: i32,
}

#[derive(Debug)]
pub enum PlayerFacing {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walk,
    Attack,
}

impl Player {
    pub fn new(w_width: i32, w_height: i32) -> Player {
        Player {
            position: normalize_coordinate(w_width, w_height),
            previous_position: normalize_coordinate(w_width, w_height),
            state: PlayerState::Idle,
            attack: 1,
            attack_range: 1,
            facing: PlayerFacing::South,
        }
    }

    pub fn draw(&self, d: &RaylibDrawHandle) {
        todo!()
    }

    pub fn save_previous_position(&mut self) {
        self.previous_position = self.position;
    }

    pub fn revert_position(&mut self) {
        self.position = self.previous_position;
    }
}
