use raylib::math::Rectangle;

use crate::state_manager::{Direction, EnemyState, PlayerState};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub prev_x: f32,
    pub prev_y: f32,
}

pub trait Movable {
    fn move_to(&mut self, dx: f32, dy: f32, speed: f32);
}

pub trait Breakable {
    fn destroy(&self);
}

pub trait Entities {
    fn instantiate() -> Self;
}

#[derive(Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug)]
pub struct Combat {
    pub attack: i32,
    range: i32,
    pub attack_timer: i32,
    pub attack_cooldown: i32,
}

#[derive(Debug)]
pub struct Player {
    pub pos: Position,
    health: Health,
    pub combat: Combat,
    pub state: PlayerState,
    pub facing: Direction,
    pub hitbox: Rectangle,
}

#[derive(Debug)]
pub struct Enemy {
    pub pos: Position,
    pub health: Health,
    pub combat: Combat,
    pub state: EnemyState,
    pub facing: Direction,
    pub hitbox: Rectangle,
}

#[derive(Debug)]
pub struct Crate {
    pub pos: Position,
    health: Health,
}

impl Movable for Player {
    fn move_to(&mut self, dx: f32, dy: f32, speed: f32) {
        self.pos.x += dx * speed;
        self.pos.y += dy * speed;
        self.hitbox.x = self.pos.x;
        self.hitbox.y = self.pos.y;
    }
}

impl Movable for Enemy {
    fn move_to(&mut self, dx: f32, dy: f32, speed: f32) {}
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
                attack: 34,
                range: 1,
                attack_timer: 12,
                attack_cooldown: 30,
            },
            hitbox: Rectangle {
                x,
                y,
                width: 14.0,
                height: 14.0,
            },
            facing: Direction::South,
        }
    }

    pub fn weapon_hitbox(&self) -> Rectangle {
        Rectangle {
            x: match self.facing {
                Direction::North => self.pos.x,
                Direction::East => self.pos.x + 14.0,
                Direction::South => self.pos.x,
                Direction::West => self.pos.x - 14.0,
            },
            y: match self.facing {
                Direction::North => self.pos.y - 14.0,
                Direction::East => self.pos.y,
                Direction::South => self.pos.y + 14.0,
                Direction::West => self.pos.y,
            },
            width: 14.0,
            height: 14.0,
        }
    }
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            state: EnemyState::Idle,
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
                attack_timer: 12,
                attack_cooldown: 30,
            },
            hitbox: Rectangle {
                x,
                y,
                width: 15.0,
                height: 15.0,
            },
            facing: Direction::South,
        }
    }
}
