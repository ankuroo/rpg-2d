use bevy::prelude::*;

use crate::components::*;
use crate::player::status::Exhausted;
use crate::player::{player::*, state::*};

#[derive(Event)]
pub struct AttackEvent;

#[derive(Component)]
pub struct AttackTimer(pub Timer);

pub fn attack_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut AttackTimer, &mut PlayerState), With<Player>>
) {
    for (entity, mut timer, mut state) in player_query.iter_mut() {

        timer.0.tick(time.delta());

        if timer.0.finished() {
            *state = PlayerState::Idle; 
            commands.entity(entity).remove::<AttackTimer>();
        }

    }
}

pub fn handle_attack_inputs(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Stamina, &mut PlayerState, Option<&Exhausted>), With<Player>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut attack_writer: EventWriter<AttackEvent>,
) {
    
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok((entity, mut stamina, mut state, exhausted)) = player_query.single_mut() {
            if exhausted.is_none() && !state.is_attacking() {
                stamina.deplete(5.0);
                attack_writer.write(AttackEvent);
                *state = PlayerState::Attacking;
                commands.entity(entity).insert(AttackTimer(Timer::from_seconds(0.3, TimerMode::Once)));
            }
        }
    }
}