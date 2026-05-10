mod constants;
mod entities;
mod map;
mod state_manager;
mod utils;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use raylib::{ffi::Rectangle, prelude::*};

use crate::{
    constants::{MOVE_SPEED, TILE_HEIGHT, TILE_WIDTH, TITLE, WINDOW_HEIGHT, WINDOW_WIDTH},
    entities::{Enemy, Movable, Player},
    state_manager::{Direction, Game, GlobalAction, Kind, PlayerState},
    utils::{breakdown_tiles, get_facing},
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(TITLE)
        .build();

    let middle_x = (WINDOW_WIDTH / 2) as f32;
    let middle_y = (WINDOW_HEIGHT / 2) as f32;
    let mut g = Game::new();
    let mut p = Player::new(middle_x, middle_y);
    let mut map_file = PathBuf::new();
    map_file.push(env!("CARGO_MANIFEST_DIR"));
    map_file.push("assets");
    map_file.push("maps");
    map_file.push("base2.json");
    let map = tiled_json_rs::Map::load_from_file(&map_file.as_path()).expect("Cannot open!");
    g.map.push(map.clone());
    let tile_arr = breakdown_tiles(&map.tile_sets);
    g.tiles = tile_arr;
    let mut player_texture_path = PathBuf::new();
    player_texture_path.push(env!("CARGO_MANIFEST_DIR"));
    player_texture_path.push("assets");
    player_texture_path.push("mystic-woods");
    player_texture_path.push("sprite_sheet_.png");
    let player_texture = rl
        .load_texture(&thread, player_texture_path.to_str().unwrap())
        .unwrap();
    // NOTE: index 1 always for player
    g.character_textures.push(player_texture);
    for tileset in &map.tile_sets {
        let map_dir = map_file.parent().unwrap();
        let image_path = map_dir.join(&tileset.image).canonicalize().unwrap();
        let t = rl
            .load_texture(&thread, image_path.to_str().unwrap())
            .unwrap();
        g.textures.push(t);
    }

    let mut e: Vec<Enemy> = vec![];
    let mut pending_actions: Vec<GlobalAction> = vec![GlobalAction::None];

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        handle_input(&rl, &mut g, &mut p, &mut pending_actions);

        update(&rl, &mut pending_actions, &mut p, &mut e);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &mut g, &mut p, &mut e);
    }
}

fn try_to_spawn_enemies(e: &mut Vec<Enemy>) {
    // WARNING: this assumes linux OS. fix later!
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf).unwrap();
    let x: i32 = buf[0].into();
    let y: i32 = buf[1].into();
    let test_subject = Enemy::new((x * 3) as f32, (y * 3) as f32);
    if e.len() < 3 {
        e.push(test_subject);
    }
}

fn handle_input(
    rl: &RaylibHandle,
    g: &mut Game,
    p: &mut Player,
    pending_actions: &mut Vec<GlobalAction>,
) {
    let mut any_input_pressed = false;
    let (dx, dy) = player_direction(rl);
    if dx != 0.0 || dy != 0.0 {
        any_input_pressed = true;
        pending_actions.push(GlobalAction::Move {
            entity: Kind::Player,
            dx,
            dy,
        });
    }

    if attack_key_pressed(rl) && p.combat.attack_cooldown == 0 {
        any_input_pressed = true;
        pending_actions.push(GlobalAction::Attack {
            entity: Kind::Player,
        });
    }

    if !any_input_pressed {
        pending_actions.push(GlobalAction::None);
    }
}

fn update(
    rl: &RaylibHandle,
    pending_actions: &mut Vec<GlobalAction>,
    p: &mut Player,
    e: &mut Vec<Enemy>,
) {
    // Save position before attempting movement
    p.pos.prev_x = p.pos.x;
    p.pos.prev_y = p.pos.y;
    let weapon_hitbox = p.weapon_hitbox();
    try_to_spawn_enemies(e);

    for action in &mut *pending_actions {
        match action {
            GlobalAction::Move {
                entity: Kind::Player,
                dx,
                dy,
            } => {
                let Some(facing) = get_facing(*dx, *dy) else {
                    continue;
                };
                if p.state != PlayerState::Attack {
                    // Clamp position based on movement direction before updating facing
                    if *dx < 0.0 && p.pos.x <= TILE_WIDTH as f32 {
                        p.pos.x = 16.0;
                    }
                    if *dx > 0.0 && p.pos.x >= (WINDOW_WIDTH - 32) as f32 {
                        p.pos.x = (WINDOW_WIDTH - (p.hitbox.width + 16.0) as i32) as f32;
                    }
                    if *dy < 0.0 && p.pos.y <= TILE_HEIGHT as f32 {
                        p.pos.y = 16.0;
                    }
                    if *dy > 0.0
                        && p.pos.y >= (WINDOW_HEIGHT - (p.hitbox.height + 28.0) as i32) as f32
                    {
                        p.pos.y = (WINDOW_HEIGHT - (p.hitbox.height + 28.0) as i32) as f32;
                    }
                    p.facing = facing;
                    if p.combat.attack_timer <= 0 {
                        p.state = PlayerState::Walk;
                    }
                    p.move_to(*dx, *dy, MOVE_SPEED * rl.get_frame_time());
                }
            }
            GlobalAction::Move {
                entity: Kind::Enemy,
                dx,
                dy,
            } => {
                // try to move toward player
                for enemy in e.iter_mut() {
                    enemy.move_to(*dx, *dy, MOVE_SPEED * rl.get_frame_time());
                }
            }
            GlobalAction::Move {
                entity: Kind::Neutral,
                dx: _,
                dy: _,
            } => (),
            GlobalAction::Attack {
                entity: Kind::Player,
            } => {
                p.state = PlayerState::Attack;
                p.combat.attack_timer = 13;
                p.combat.attack_cooldown = 30; // 0.5s cooldown at 60fps
            }
            GlobalAction::Attack {
                entity: Kind::Enemy,
            } => {}
            GlobalAction::Attack {
                entity: Kind::Neutral,
            } => {}
            GlobalAction::None => {
                if p.combat.attack_timer <= 0 {
                    p.state = PlayerState::Idle;
                }
            }
        }
    }
    pending_actions.clear();

    // Decrement attack timer each frame
    if p.combat.attack_timer > 0 {
        p.combat.attack_timer -= 1;
    }

    if p.combat.attack_timer == 0 {
        p.state = PlayerState::Idle;
    }

    // Decrement attack cooldown each frame
    if p.combat.attack_cooldown > 0 {
        p.combat.attack_cooldown -= 1;
    }

    for enemy in e.iter_mut() {
        if weapon_hitbox.check_collision_recs(&enemy.hitbox)
            && p.state == PlayerState::Attack
            && p.combat.attack_timer == 12
        {
            enemy.health.current -= p.combat.attack;
        }
    }
    let mut index_enemies_to_remove: Vec<usize> = vec![];
    for (index, enemy) in e.iter_mut().enumerate() {
        // Resolve collisions with enemies
        if enemy.hitbox.x <= 0.0
            || enemy.hitbox.x >= (WINDOW_WIDTH - (TILE_WIDTH * 2)) as f32
            || enemy.hitbox.y <= 0.0
            || enemy.hitbox.y >= (WINDOW_HEIGHT - (TILE_HEIGHT * 2)) as f32
            || enemy.health.current <= 0
        {
            index_enemies_to_remove.push(index);
        }
        if p.hitbox.check_collision_recs(&enemy.hitbox) {
            p.pos.x = p.pos.prev_x;
            p.pos.y = p.pos.prev_y;
            p.hitbox.x = p.pos.x;
            p.hitbox.y = p.pos.y;
            break;
        }
    }
    for i in index_enemies_to_remove.iter().rev() {
        e.remove(*i);
    }
}

fn player_direction(rl: &RaylibHandle) -> (f32, f32) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        dy -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        dx += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        dy += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        dx -= 1.0;
    }
    if dx != 0.0 && dy != 0.0 {
        dx *= 0.707;
        dy *= 0.707;
    }

    (dx, dy)
}

fn attack_key_pressed(rl: &RaylibHandle) -> bool {
    rl.is_key_pressed(KeyboardKey::KEY_SPACE)
}

fn draw(d: &mut RaylibDrawHandle, g: &mut Game, p: &mut Player, e: &mut Vec<Enemy>) {
    draw_world(d, g);

    draw_entities(d, g, p, e);

    draw_ui(d, g, p);
}

fn draw_world(d: &mut RaylibDrawHandle, g: &mut Game) {
    d.clear_background(Color::WHITE);

    // TODO: this causes map overlap
    for map in g.map.iter() {
        for map_layer in &map.layers {
            match &map_layer.layer_type {
                tiled_json_rs::LayerType::ImageLayer(_image) => {
                    todo!()
                }
                tiled_json_rs::LayerType::Group { layers: _ } => {
                    todo!()
                }
                tiled_json_rs::LayerType::ObjectGroup(_obj) => {
                    todo!()
                }
                tiled_json_rs::LayerType::TileLayer(tile_layer) => {
                    for (i, gid) in tile_layer.data.iter().enumerate() {
                        if *gid == 0 {
                            continue;
                        }
                        let tile_x = (i as u32 % tile_layer.width) * TILE_WIDTH as u32;
                        let tile_y = (i as u32 / tile_layer.width) * TILE_HEIGHT as u32;

                        if let Some(&rect) = g.tiles.get(*gid as usize) {
                            for (idx, tileset) in map.tile_sets.iter().enumerate() {
                                if *gid >= tileset.first_gid
                                    && *gid < tileset.first_gid + tileset.tile_count
                                {
                                    if let Some(texture) = g.textures.get(idx) {
                                        d.draw_texture_rec(
                                            texture,
                                            rect,
                                            Vector2::new(tile_x as f32, tile_y as f32),
                                            Color::WHITE,
                                        );
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn draw_entities(d: &mut RaylibDrawHandle, g: &mut Game, p: &mut Player, e: &mut Vec<Enemy>) {
    match p.state {
        PlayerState::Idle => {
            // animate idle
        }
        PlayerState::Attack => {
            // animate attack
        }
        PlayerState::Walk => {
            // animate move
        }
        PlayerState::Dead => {
            // animate death
        }
    }

    // player
    match p.facing {
        Direction::North => d.draw_texture_rec(
            &g.character_textures[0],
            Rectangle {
                x: 194.0,
                y: 5.0,
                width: 16.0,
                height: 20.0,
            },
            Vector2 {
                x: p.pos.x,
                y: p.pos.y,
            },
            Color::WHITE,
        ),
        Direction::East => d.draw_texture_rec(
            &g.character_textures[0],
            Rectangle {
                x: 92.0,
                y: 5.0,
                width: 16.0,
                height: 22.0,
            },
            Vector2 {
                x: p.pos.x,
                y: p.pos.y,
            },
            Color::WHITE,
        ),
        Direction::South => d.draw_texture_rec(
            &g.character_textures[0],
            Rectangle {
                x: 2.0,
                y: 5.0,
                width: 16.0,
                height: 20.0,
            },
            Vector2 {
                x: p.pos.x,
                y: p.pos.y,
            },
            Color::WHITE,
        ),
        Direction::West => d.draw_texture_rec(
            &g.character_textures[0],
            Rectangle {
                x: 92.0,
                y: 5.0,
                width: -16.0,
                height: 22.0,
            },
            Vector2 {
                x: p.pos.x,
                y: p.pos.y,
            },
            Color::WHITE,
        ),
    };
    d.draw_rectangle_lines_ex(p.hitbox, 1.0, Color::RED);

    // enemies
    for enemy in e.iter() {
        d.draw_rectangle_lines_ex(enemy.hitbox, 1.0, Color::ORANGE);
    }

    // animate sword swing
    let weapon_hitbox = p.weapon_hitbox();
    if p.combat.attack_timer > 0 && p.state == PlayerState::Attack {
        d.draw_rectangle_lines_ex(weapon_hitbox, 1.0, Color::RED);
    }
}

fn draw_ui(d: &mut RaylibDrawHandle, g: &mut Game, p: &mut Player) {
    // TODO: draw UI
}
