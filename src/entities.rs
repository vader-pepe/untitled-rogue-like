use raylib::ffi::KeyboardKey;

use crate::{
    state_manager::{Direction, EnemyState, PlayerState},
    utils::get_facing,
};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub prev_x: f32,
    pub prev_y: f32,
}

pub trait Movable {
    fn moving(&mut self, dx: f32, dy: f32, speed: f32);
}

pub trait Breakable {
    fn destroy(&self);
}

#[derive(Debug)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Debug)]
struct Combat {
    attack: i32,
    range: i32,
}

#[derive(Debug)]
pub struct Player {
    pub pos: Position,
    health: Health,
    combat: Combat,
    pub state: PlayerState,
    pub facing: Direction,
}

#[derive(Debug)]
pub struct Enemy {
    pub pos: Position,
    health: Health,
    combat: Combat,
    state: EnemyState,
    facing: Direction,
}

#[derive(Debug)]
pub struct Crate {
    pub pos: Position,
    health: Health,
}

impl Movable for Player {
    fn moving(&mut self, dx: f32, dy: f32, speed: f32) {
        self.pos.x += dx * speed;
        self.pos.y += dy * speed;
    }
}

impl Movable for Enemy {
    fn moving(&mut self, dx: f32, dy: f32, speed: f32) {}
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            state: PlayerState::Idle,
            pos: Position {
                x,
                y,
                prev_x: x,
                prev_y: y,
            },
            health: Health {
                current: 100,
                max: 100,
            },
            combat: Combat {
                attack: 1,
                range: 1,
            },
            facing: Direction::South,
        }
    }

    pub fn attack(&mut self, key: KeyboardKey) {
        let mut any_key_down = false;
        if key == KeyboardKey::KEY_SPACE {
            any_key_down = true;
            self.state = PlayerState::Attack;
        }
        if !any_key_down {
            self.state = PlayerState::Idle;
        }
    }

    pub fn save_previous_position(&mut self) {
        self.pos.prev_x = self.pos.x;
        self.pos.prev_y = self.pos.y;
    }

    pub fn revert_position(&mut self) {
        self.pos.x = self.pos.prev_x;
        self.pos.y = self.pos.prev_y;
    }
}
