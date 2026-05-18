mod anim;
mod constants;
mod entities;
mod inputs;
mod map;
mod state_manager;
mod utils;
mod world;

use std::path::PathBuf;

use raylib::prelude::*;

use crate::{
    anim::{Anim, AnimFrame, AnimState},
    constants::{MOVE_SPEED, TILE_HEIGHT, TILE_WIDTH, TITLE, WINDOW_HEIGHT, WINDOW_WIDTH},
    entities::{Enemy, Movable, Player},
    inputs::handle_input,
    state_manager::{Direction, Game, GlobalAction, Kind, PlayerState},
    utils::{breakdown_tiles, get_facing},
    world::draw_world,
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(TITLE)
        .build();

    let middle_x = (WINDOW_WIDTH / 2) as f32;
    let middle_y = (WINDOW_HEIGHT / 2) as f32;
    let mut g = Game::new();
    let idle_frames_south = vec![
        AnimFrame {
            rec: Rectangle::new(2.5, 5.0, 13.0, 21.0),
            duration: 0.1,
        },
        AnimFrame {
            rec: Rectangle::new(17.5, 5.0, 13.0, 21.0),
            duration: 0.1,
        },
        AnimFrame {
            rec: Rectangle::new(32.5, 5.0, 13.0, 21.0),
            duration: 0.1,
        },
        AnimFrame {
            rec: Rectangle::new(47.6, 5.0, 13.0, 21.0),
            duration: 0.1,
        },
        AnimFrame {
            rec: Rectangle::new(62.6, 5.0, 13.0, 20.0),
            duration: 0.1,
        },
        AnimFrame {
            rec: Rectangle::new(77.6, 5.0, 13.0, 20.0),
            duration: 0.1,
        },
    ];

    let idle_anim_south = Anim {
        frames: idle_frames_south.clone(),
        looping: true,
    };

    let player_anim_idle_south = AnimState::new(idle_anim_south.clone());

    let mut p = Player::new(middle_x, middle_y, vec![player_anim_idle_south.clone()]);
    p.animations.push(player_anim_idle_south);
    // TODO: create a big texture atlas so texture load only happened once
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
    let mut k = String::new();
    k.push_str("player");
    g.character_textures.insert(k, player_texture);
    for tileset in &map.tile_sets {
        let map_dir = map_file.parent().unwrap();
        let image_path = map_dir.join(&tileset.image).canonicalize().unwrap();
        let t = rl
            .load_texture(&thread, image_path.to_str().unwrap())
            .unwrap();
        g.map_textures.push(t);
    }

    let mut e: Vec<Enemy> = vec![];
    let mut pending_actions: Vec<GlobalAction> = vec![GlobalAction::None];

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let delta = rl.get_frame_time();
        g.game_time.delta = delta;
        g.game_time.time_since_start += delta;

        handle_input(&rl, &mut g, &mut p, &mut pending_actions);

        update(&mut pending_actions, &mut g, &mut p, &mut e);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &mut g, &mut p, &mut e);
    }
}

fn try_to_spawn_enemies(enemies: &mut Vec<Enemy>) {
    // WARNING: this assumes linux OS. fix later!
    // let mut f = File::open("/dev/urandom").unwrap();
    // let mut buf = [0u8; 2];
    // f.read_exact(&mut buf).unwrap();
    // let x: i32 = buf[0].into();
    // let y: i32 = buf[1].into();
    // let test_subject = Enemy::new((x * 3) as f32, (y * 3) as f32);
    // if enemies.len() < 3 {
    //     enemies.push(test_subject);
    // }
}

fn update(
    pending_actions: &mut Vec<GlobalAction>,
    game_instance: &mut Game,
    player: &mut Player,
    enemies: &mut Vec<Enemy>,
) {
    // Save position before attempting movement
    player.pos.prev_x = player.pos.x;
    player.pos.prev_y = player.pos.y;
    let weapon_hitbox = player.weapon_hitbox();
    try_to_spawn_enemies(enemies);
    for animation in player.animations.iter_mut() {
        animation.update(game_instance.game_time.delta);
    }

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
                if player.state != PlayerState::Attack {
                    // Clamp position based on movement direction before updating facing
                    if *dx < 0.0 && player.pos.x <= TILE_WIDTH as f32 {
                        player.pos.x = 16.0;
                    }
                    if *dx > 0.0 && player.pos.x >= (WINDOW_WIDTH - 32) as f32 {
                        player.pos.x = (WINDOW_WIDTH - (player.hitbox.width + 16.0) as i32) as f32;
                    }
                    if *dy < 0.0 && player.pos.y <= TILE_HEIGHT as f32 {
                        player.pos.y = 16.0;
                    }
                    if *dy > 0.0
                        && player.pos.y
                            >= (WINDOW_HEIGHT - (player.hitbox.height + 28.0) as i32) as f32
                    {
                        player.pos.y =
                            (WINDOW_HEIGHT - (player.hitbox.height + 28.0) as i32) as f32;
                    }
                    player.facing = facing;
                    if player.combat.attack_timer <= 0 {
                        player.state = PlayerState::Walk;
                    }
                    // TODO: get frame time working
                    player.accelerate(*dx, *dy, MOVE_SPEED * game_instance.game_time.delta);
                }
            }
            GlobalAction::Move {
                entity: Kind::Enemy,
                dx,
                dy,
            } => {
                // try to move toward player
                for enemy in enemies.iter_mut() {
                    // TODO: get frame time working
                    enemy.accelerate(*dx, *dy, MOVE_SPEED);
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
                player.state = PlayerState::Attack;
                player.combat.attack_timer = 13;
                player.combat.attack_cooldown = 30; // 0.5s cooldown at 60fps
            }
            GlobalAction::Attack {
                entity: Kind::Enemy,
            } => {}
            GlobalAction::Attack {
                entity: Kind::Neutral,
            } => {}
            GlobalAction::None => {
                if player.combat.attack_timer <= 0 {
                    player.state = PlayerState::Idle;
                }
            }
        }
    }
    pending_actions.clear();

    // Decrement attack timer each frame
    if player.combat.attack_timer > 0 {
        player.combat.attack_timer -= 1;
    }

    if player.combat.attack_timer == 0 {
        player.state = PlayerState::Idle;
    }

    // Decrement attack cooldown each frame
    if player.combat.attack_cooldown > 0 {
        player.combat.attack_cooldown -= 1;
    }

    for enemy in enemies.iter_mut() {
        if weapon_hitbox.check_collision_recs(&enemy.hitbox)
            && player.state == PlayerState::Attack
            && player.combat.attack_timer == 12
        {
            enemy.health.current -= player.combat.attack;
        }
    }
    let mut index_enemies_to_remove: Vec<usize> = vec![];
    for (index, enemy) in enemies.iter_mut().enumerate() {
        // Resolve collisions with enemies
        if enemy.hitbox.x <= 0.0
            || enemy.hitbox.x >= (WINDOW_WIDTH - (TILE_WIDTH * 2)) as f32
            || enemy.hitbox.y <= 0.0
            || enemy.hitbox.y >= (WINDOW_HEIGHT - (TILE_HEIGHT * 2)) as f32
            || enemy.health.current <= 0
        {
            index_enemies_to_remove.push(index);
        }
        if player.hitbox.check_collision_recs(&enemy.hitbox) {
            player.pos.x = player.pos.prev_x;
            player.pos.y = player.pos.prev_y;
            player.hitbox.x = player.pos.x;
            player.hitbox.y = player.pos.y;
            break;
        }
    }
    for i in index_enemies_to_remove.iter().rev() {
        enemies.remove(*i);
    }
}

fn draw(
    draw_handle: &mut RaylibDrawHandle,
    game_instance: &mut Game,
    player: &mut Player,
    enemies: &mut Vec<Enemy>,
) {
    draw_world(draw_handle, game_instance);

    draw_entities(draw_handle, game_instance, player, enemies);

    draw_ui(draw_handle, game_instance);
}

fn draw_entities(
    draw_handle: &mut RaylibDrawHandle,
    game_instance: &mut Game,
    player: &mut Player,
    enemies: &mut Vec<Enemy>,
) {
    match player.state {
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
    match player.facing {
        Direction::North => draw_handle.draw_texture_rec(
            &game_instance.character_textures.get("player").unwrap(),
            Rectangle {
                x: 194.0,
                y: 5.0,
                width: 16.0,
                height: 20.0,
            },
            Vector2 {
                x: player.pos.x,
                y: player.pos.y,
            },
            Color::WHITE,
        ),
        Direction::East => draw_handle.draw_texture_rec(
            &game_instance.character_textures.get("player").unwrap(),
            Rectangle {
                x: 92.0,
                y: 5.0,
                width: 16.0,
                height: 22.0,
            },
            Vector2 {
                x: player.pos.x,
                y: player.pos.y,
            },
            Color::WHITE,
        ),
        Direction::South => draw_handle.draw_texture_rec(
            &game_instance.character_textures.get("player").unwrap(),
            player.animations[0].current_rect(),
            Vector2 {
                x: player.pos.x,
                y: player.pos.y,
            },
            Color::WHITE,
        ),
        Direction::West => draw_handle.draw_texture_rec(
            &game_instance.character_textures.get("player").unwrap(),
            Rectangle {
                x: 92.0,
                y: 5.0,
                width: -16.0,
                height: 22.0,
            },
            Vector2 {
                x: player.pos.x,
                y: player.pos.y,
            },
            Color::WHITE,
        ),
    };
    draw_handle.draw_rectangle_lines_ex(player.hitbox, 1.0, Color::RED);

    // enemies
    for enemy in enemies.iter() {
        draw_handle.draw_rectangle_lines_ex(enemy.hitbox, 1.0, Color::ORANGE);
    }

    // animate sword swing
    let weapon_hitbox = player.weapon_hitbox();
    if player.combat.attack_timer > 0 && player.state == PlayerState::Attack {
        draw_handle.draw_rectangle_lines_ex(weapon_hitbox, 1.0, Color::RED);
    }
}

fn draw_ui(draw_handle: &mut RaylibDrawHandle, game_instance: &mut Game) {
    // TODO: draw UI
}
