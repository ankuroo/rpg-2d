use bevy::prelude::*;

use crate::components::*;
use crate::player::*;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
    Exhausted,
    Dead
}

fn is_interrupting_state(state: PlayerState) -> bool {
    state == PlayerState::Attacking
}

pub fn update_state(
    mut player_query: Query<(&mut PlayerState, &Velocity, &Health, &mut Stamina), With<Player>>
) {

    if let Ok((mut state, velocity, health, stamina)) = player_query.single_mut() {

        if !health.is_alive() {
            *state = PlayerState::Dead;
            return
        }

        if is_interrupting_state(*state) {
            return
        }

        if stamina.is_depleted() {
            *state = PlayerState::Exhausted;
            return;
        }

        if stamina.is_full() && *state == PlayerState::Exhausted {
            if velocity.0 != Vec3::ZERO {
                *state = PlayerState::Walking;
            } else {
                *state = PlayerState::Idle;
            }
        }

        if velocity.0 != Vec3::ZERO {
            *state = PlayerState::Walking;
        } else {
            *state = PlayerState::Idle;
        }

    }

}