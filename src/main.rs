mod constants;
mod entities;
mod state_manager;
mod utils;

use raylib::prelude::*;

use crate::{
    constants::{MOVE_SPEED, TITLE, WINDOW_HEIGHT, WINDOW_WIDTH},
    entities::{Enemy, Movable, Player, Position},
    state_manager::{Game, GameState, GlobalAction, Kind, PlayerState, try_to_move},
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

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // TODO: implement all of this:
        handle_input(&rl, &mut g, &mut p, &mut pending_actions);
        update(&rl, &mut pending_actions, &mut p);
        // draw_world();
        // draw_entities();
        // draw_ui();

        // p.save_previous_position();

        // process_inputs(&rl, &mut p);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &mut g, &mut p);

        print!("{:?}\n", p);
    }
}

fn handle_input(
    rl: &RaylibHandle,
    g: &mut Game,
    p: &mut Player,
    pending_actions: &mut Vec<GlobalAction>,
) {
    let (dx, dy) = movement_keys_pressed(rl);
    pending_actions.push(GlobalAction::Move {
        entity: Kind::Player,
        dx,
        dy,
    });

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
            GlobalAction::Attack => p.state = PlayerState::Attack,
            GlobalAction::Wait => (),
            GlobalAction::None => (),
        }
    }
    p.state = PlayerState::Idle;
    pending_actions.clear();
}

fn movement_keys_pressed(rl: &RaylibHandle) -> (f32, f32) {
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
    let box_a = Rectangle {
        x: p.pos.x,
        y: p.pos.y,
        width: 10.0,
        height: 10.0,
    };
    let window_collider = Rectangle {
        x: 2.5,
        y: 5.0,
        height: (WINDOW_HEIGHT - 10) as f32,
        width: (WINDOW_WIDTH - 5) as f32,
    };

    d.clear_background(Color::WHITE);
    d.draw_rectangle_lines_ex(window_collider, 1.0, Color::RED);

    // PLAYER DRAWING
    {
        d.draw_text("#", p.pos.x as i32, p.pos.y as i32, 15, Color::BLACK);
        // d.draw_rectangle_rec(box_a, Color::YELLOW);
        // match window_collider.get_collision_rec(&box_a) {
        //     None => {
        //         p.revert_position();
        //         g.state = GameState::GameOver;
        //     }
        //     _ => (),
        // }
        // match p.facing {
        //     PlayerFacing::South => {
        //         if p.state == PlayerState::Attack {
        //             d.draw_text(
        //                 "|",
        //                 (p.position.x + 2.0) as i32,
        //                 (p.position.y + 10.0) as i32,
        //                 2,
        //                 Color::BLACK,
        //             );
        //         }
        //     }
        //     PlayerFacing::East => {
        //         if p.state == PlayerState::Attack {
        //             d.draw_text(
        //                 "--",
        //                 (p.position.x + 10.0) as i32,
        //                 (p.position.y) as i32,
        //                 2,
        //                 Color::BLACK,
        //             );
        //         }
        //     }
        //     PlayerFacing::North => {
        //         if p.state == PlayerState::Attack {
        //             d.draw_text(
        //                 "|",
        //                 (p.position.x + 2.0) as i32,
        //                 (p.position.y - 10.0) as i32,
        //                 2,
        //                 Color::BLACK,
        //             );
        //         }
        //     }
        //     PlayerFacing::West => {
        //         if p.state == PlayerState::Attack {
        //             d.draw_text(
        //                 "--",
        //                 (p.position.x - 15.0) as i32,
        //                 (p.position.y) as i32,
        //                 2,
        //                 Color::BLACK,
        //             );
        //         }
        //     }
        // }
    }
}
