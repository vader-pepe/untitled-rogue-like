mod entities;
mod system;
mod utils;

use raylib::prelude::*;

use crate::{
    entities::{Player, PlayerFacing, PlayerState},
    utils::Window,
};

fn main() {
    let window = Window::new(640, 480);
    let (mut rl, thread) = raylib::init()
        .size(window.width, window.height)
        .title("Hello, World")
        .build();

    let mut p = Player::new(window.width, window.height);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        let key_pressed = d.get_key_pressed();

        d.draw_text("*", p.position.x, p.position.y, 2, Color::BLACK);
        // NOTE: so character size is 10. i somehow get it right to minus it by the window height

        process_inputs(key_pressed, &mut p);
        print!("{:?}\n", p);
    }
}

fn process_inputs(d: Option<KeyboardKey>, p: &mut Player) {
    match d {
        Some(KeyboardKey::KEY_UP) => {
            p.state = PlayerState::Walk;
            p.facing = PlayerFacing::South;
            p.position.y -= 1;
            print!("UP\n")
        }
        Some(KeyboardKey::KEY_DOWN) => {
            p.state = PlayerState::Walk;
            p.facing = PlayerFacing::North;
            p.position.y += 1;
            print!("DOWN\n")
        }
        Some(KeyboardKey::KEY_LEFT) => {
            p.state = PlayerState::Walk;
            p.facing = PlayerFacing::East;
            p.position.x -= 1;
            print!("LEFT\n")
        }
        Some(KeyboardKey::KEY_RIGHT) => {
            p.state = PlayerState::Walk;
            p.facing = PlayerFacing::West;
            p.position.x += 1;
            print!("RIGHT\n")
        }
        _ => {
            p.state = PlayerState::Idle;
        }
    }
}
