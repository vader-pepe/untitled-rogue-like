use raylib::{RaylibHandle, ffi::KeyboardKey};

use crate::{Game, GlobalAction, Kind, Player, entities::Movable, state_manager::player_direction};

pub fn handle_input(
    rl_handle: &RaylibHandle,
    game_instance: &mut Game,
    player: &mut Player,
    pending_actions: &mut Vec<GlobalAction>,
) {
    let mut any_input_pressed = false;
    let (dx, dy) = player_direction(rl_handle);
    if dx != 0.0 || dy != 0.0 {
        any_input_pressed = true;
        pending_actions.push(GlobalAction::Move {
            entity: Kind::Player,
            dx,
            dy,
        });
    }

    if attack_key_pressed(rl_handle) && player.combat.attack_cooldown == 0 {
        any_input_pressed = true;
        pending_actions.push(GlobalAction::Attack {
            entity: Kind::Player,
        });
    }

    if !any_input_pressed {
        // TODO: need to revisit this
        // pending_actions.push(GlobalAction::None);
    }
}

fn attack_key_pressed(rl: &RaylibHandle) -> bool {
    rl.is_key_pressed(KeyboardKey::KEY_SPACE)
}

pub fn accelerate_entity<T: Movable>(entity: &mut T, dx: f32, dy: f32, speed: f32) {
    // TODO: handle collision somewhere else
    entity.accelerate(dx, dy, speed);
    // TODO: how to get rl.get_frame_time() here???
}
