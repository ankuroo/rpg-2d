use bevy::prelude::*;

use crate::components::*;
use crate::player::*;

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
    Dead
}

impl PlayerState {

    pub fn is_attacking(&self) -> bool {
        *self == PlayerState::Attacking
    }
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

        if velocity.0 != Vec3::ZERO {
            *state = PlayerState::Walking;
        } else {
            *state = PlayerState::Idle;
        }

    }

}