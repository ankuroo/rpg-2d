use bevy::prelude::*;

use crate::{components::*, player::Player};


pub fn update_velocity(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
){
    if let Ok(mut velocity) = player_query.single_mut() {
        let mut v = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            v += Vec3::Y;
        }
        
        if keys.pressed(KeyCode::KeyS) {
            v += Vec3::NEG_Y;
        }
        
        if keys.pressed(KeyCode::KeyA) {
            v += Vec3::NEG_X;
        }
        
        if keys.pressed(KeyCode::KeyD) {
            v += Vec3::X;
        }

        velocity.0 = v.normalize_or_zero() * 100.0;
        
    }
}

pub fn update_position(
    time: Res<Time>,
    mut player_query: Query<(&mut Position, &Velocity), With<Player>>
){
    if let Ok((mut position, velocity)) = player_query.single_mut() {
        position.0 += velocity.0 * time.delta_secs();
    }
}

pub fn sync_position_transform(
    mut player_query: Query<(&mut Transform, &Position), With<Player>>
){
    if let Ok((mut transform, position)) = player_query.single_mut() {
        transform.translation = position.0;
    }
}