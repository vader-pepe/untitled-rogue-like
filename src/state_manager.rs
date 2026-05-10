use raylib::{ffi::Rectangle, texture::Texture2D};
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
pub struct Game {
    pub state: GameState,
    pub textures: Vec<Texture2D>,
    pub character_textures: Vec<Texture2D>,
    pub map: Vec<Map>,
    pub tiles: Vec<Rectangle>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            // TODO: show menu first idiot
            state: GameState::Playing,
            textures: vec![],
            character_textures: vec![],
            map: vec![],
            tiles: vec![],
        }
    }
}
