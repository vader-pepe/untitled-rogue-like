mod constants;
mod entities;
mod system;
mod utils;

use raylib::prelude::*;

use crate::{
    constants::{MOVE_SPEED, TITLE, WINDOW_HEIGHT, WINDOW_WIDTH},
    entities::{Player, PlayerFacing, PlayerState},
    utils::Window,
};

fn main() {
    let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let (mut rl, thread) = raylib::init()
        .size(window.width, window.height)
        .title(TITLE)
        .build();

    let mut p = Player::new(window.width, window.height);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        process_inputs(&rl, &mut p);

        // NOTE: draw call should be
        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &mut p);

        print!("{:?}\n", p);
    }
}

fn draw(d: &mut RaylibDrawHandle, p: &mut Player) {
    d.clear_background(Color::WHITE);

    // PLAYER DRAWING
    {
        d.draw_text(
            "*",
            (p.position.x) as i32,
            (p.position.y) as i32,
            2,
            Color::BLACK,
        );
        match p.facing {
            PlayerFacing::South => {
                if p.state == PlayerState::Attack {
                    d.draw_text(
                        "|",
                        (p.position.x + 2.0) as i32,
                        (p.position.y + 10.0) as i32,
                        2,
                        Color::BLACK,
                    );
                }
            }
            PlayerFacing::East => {
                if p.state == PlayerState::Attack {
                    d.draw_text(
                        "--",
                        (p.position.x + 10.0) as i32,
                        (p.position.y) as i32,
                        2,
                        Color::BLACK,
                    );
                }
            }
            PlayerFacing::North => {
                if p.state == PlayerState::Attack {
                    d.draw_text(
                        "|",
                        (p.position.x + 2.0) as i32,
                        (p.position.y - 10.0) as i32,
                        2,
                        Color::BLACK,
                    );
                }
            }
            PlayerFacing::West => {
                if p.state == PlayerState::Attack {
                    d.draw_text(
                        "--",
                        (p.position.x - 15.0) as i32,
                        (p.position.y) as i32,
                        2,
                        Color::BLACK,
                    );
                }
            }
        }
    }
}

fn process_inputs(rl: &RaylibHandle, p: &mut Player) {
    let mut any_key_down = false;
    let mut dx = 0.0;
    let mut dy = 0.0;

    // TODO: should call 'move'
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        dx += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        dx -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        dy += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        dy -= 1.0;
    }

    if dx != 0.0 && dy != 0.0 {
        dx *= 0.707; // 1/√2
        dy *= 0.707;
    }

    if dx != 0.0 || dy != 0.0 {
        p.state = PlayerState::Walk;
        if let Some(facing) = get_facing(dx, dy) {
            p.facing = facing;
        }
        p.position.x += dx * MOVE_SPEED * rl.get_frame_time();
        p.position.y += dy * MOVE_SPEED * rl.get_frame_time();
    }

    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        any_key_down = true;
        p.state = PlayerState::Attack;
        println!("ATTACK");
    }

    if !any_key_down {
        p.state = PlayerState::Idle;
    }
}

fn get_facing(dx: f32, dy: f32) -> Option<PlayerFacing> {
    match (dx, dy) {
        (0.0, -1.0) => Some(PlayerFacing::North),
        (1.0, 0.0) => Some(PlayerFacing::East),
        (0.0, 1.0) => Some(PlayerFacing::South),
        (-1.0, 0.0) => Some(PlayerFacing::West),
        _ => None, // no movement
    }
}
