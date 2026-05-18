use std::collections::HashMap;

use raylib::{
    RaylibHandle,
    ffi::{KeyboardKey, Rectangle},
    texture::Texture2D,
};
use tiled_json_rs::Map;

#[derive(Debug)]
pub enum GlobalAction {
    Move { entity: Kind, dx: f32, dy: f32 },
    Attack { entity: Kind },
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
pub struct GameTime {
    pub delta: f32,
    pub time_since_start: f32,
}

#[derive(Debug)]
pub struct Game {
    pub game_time: GameTime,
    pub state: GameState,
    pub map_textures: Vec<Texture2D>,
    pub character_textures: HashMap<String, Texture2D>,
    pub map: Vec<Map>,
    pub tiles: Vec<Rectangle>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_time: GameTime {
                delta: 0.0,
                time_since_start: 0.0,
            },
            // TODO: show menu first idiot
            state: GameState::Playing,
            map_textures: vec![],
            character_textures: HashMap::new(),
            map: vec![],
            tiles: vec![],
        }
    }
}

pub fn player_direction(rl: &RaylibHandle) -> (f32, f32) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        dy -= 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        dx += 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        dy += 1.0;
    } else if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        dx -= 1.0;
    }
    // NOTE: this is for diagonal movement.
    // currently disabled
    // if dx != 0.0 && dy != 0.0 {
    //     dx *= 0.707;
    //     dy *= 0.707;
    // }

    (dx, dy)
}
