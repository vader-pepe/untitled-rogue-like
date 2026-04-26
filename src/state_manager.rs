use raylib::RaylibHandle;

use crate::{
    constants::MOVE_SPEED,
    entities::{Crate, Enemy, Movable, Player},
};

#[derive(Debug)]
pub enum GlobalAction {
    Move { entity: Kind, dx: f32, dy: f32 },
    Attack,
    Wait,
    None,
}

#[derive(Debug)]
pub enum Kind {
    Player,
    Enemy,
    Neutral,
}

#[derive(Debug)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
    Paused,
}

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walk,
    Attack,
    Dead,
}

#[derive(Debug)]
pub enum EnemyState {
    Idle,
    Walk,
    Chasing,
    Searching,
    Attack,
    Dead,
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            // TODO: show menu first idiot
            state: GameState::Playing,
        }
    }
}

pub fn try_to_move<T: Movable>(rl: &RaylibHandle, entity: &mut T, dx: f32, dy: f32) {
    // TODO: check if there is obstacle forward
    entity.moving(dx, dy, MOVE_SPEED * rl.get_frame_time());
}
