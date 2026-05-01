mod constants;
mod entities;
mod state_manager;
mod utils;

use std::ops::Range;

use raylib::prelude::*;

use crate::{
    constants::{MOVE_SPEED, TITLE, WINDOW_HEIGHT, WINDOW_WIDTH},
    entities::{Enemy, Movable, Player, Position},
    state_manager::{Direction, Game, GameState, GlobalAction, Kind, PlayerState, try_to_move},
    utils::{get_facing, normalize_coordinate},
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(TITLE)
        .build();

    let zero = normalize_coordinate(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut p = Player::new(zero.x, zero.y);
    let mut g = Game::new();
    let mut pending_actions: Vec<GlobalAction> = vec![GlobalAction::None];
    let world_border = Rectangle {
        x: 0.0,
        y: 0.0,
        height: WINDOW_HEIGHT as f32,
        width: WINDOW_WIDTH as f32,
    };

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        handle_input(&rl, &mut g, &mut p, &mut pending_actions);
        // println!("{:?}", pending_actions);
        println!("{:?}", p);
        update(&rl, &mut pending_actions, &mut p);
        // draw_world();
        // draw_entities();
        // draw_ui();

        // p.save_previous_position();

        // process_inputs(&rl, &mut p);

        let mut d = rl.begin_drawing(&thread);
        d.draw_rectangle_lines(
            p.hitbox.x as i32,
            p.hitbox.y as i32,
            p.hitbox.width as i32,
            p.hitbox.height as i32,
            Color::RED,
        );

        draw(&mut d, &mut g, &mut p);
    }
}

fn handle_input(
    rl: &RaylibHandle,
    g: &mut Game,
    p: &mut Player,
    pending_actions: &mut Vec<GlobalAction>,
) {
    let (mut dx, mut dy) = player_direction(rl);
    if dx != 0.0 || dy != 0.0 {
        pending_actions.push(GlobalAction::Move {
            entity: Kind::Player,
            dx,
            dy,
        });
    } else if dx == 0.0 && dy == 0.0 {
        pending_actions.push(GlobalAction::None);
    }

    if attack_key_pressed(rl) {
        pending_actions.push(GlobalAction::Attack);
    }
}

fn update(rl: &RaylibHandle, pending_actions: &mut Vec<GlobalAction>, p: &mut Player) {
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
                // Clamp position based on movement direction before updating facing
                if *dx < 0.0 && p.pos.x <= 0.0 {
                    p.pos.x = 0.0;
                }
                if *dx > 0.0 && p.pos.x >= (WINDOW_WIDTH - 15) as f32 {
                    p.pos.x = (WINDOW_WIDTH - 15) as f32;
                }
                if *dy < 0.0 && p.pos.y <= 0.0 {
                    p.pos.y = 0.0;
                }
                if *dy > 0.0 && p.pos.y >= (WINDOW_HEIGHT - 15) as f32 {
                    p.pos.y = (WINDOW_HEIGHT - 15) as f32;
                }
                p.facing = facing;
                p.state = PlayerState::Walk;
                try_to_move(rl, p, *dx, *dy);
            }
            GlobalAction::Move {
                entity: Kind::Enemy,
                dx,
                dy,
            } => (),
            GlobalAction::Move {
                entity: Kind::Neutral,
                dx,
                dy,
            } => (),
            GlobalAction::Collide {
                entity: Kind::Player,
                dx,
                dy,
            } => (),
            GlobalAction::Collide {
                entity: Kind::Enemy,
                dx,
                dy,
            } => (),
            GlobalAction::Collide {
                entity: Kind::Neutral,
                dx,
                dy,
            } => (),
            GlobalAction::Attack => p.state = PlayerState::Attack,
            GlobalAction::Wait => (),
            GlobalAction::None => (),
        }
    }

    // Transition out of Colliding if back in bounds

    pending_actions.clear();
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

fn draw(d: &mut RaylibDrawHandle, g: &mut Game, p: &mut Player) {
    d.clear_background(Color::WHITE);

    d.draw_text("#", p.pos.x as i32, p.pos.y as i32, 15, Color::RED);
    // match world_border.get_collision_rec(&player_hitbox) {
    //     Some(rec) => {
    //         println!("collision: {:?}", rec);
    //     }
    //     None => (),
    // }
}

fn into_vector2_of_math(x: f32, y: f32) -> raylib::math::Vector2 {
    raylib::math::Vector2 { x: 0.0, y: 0.0 }
}
