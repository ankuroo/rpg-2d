use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::components::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: Position,
    velocity: Velocity,
    sprite: Sprite,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (handle_inputs, move_player, sync_position_transform).chain())
        ;
    }
}

fn spawn_player(mut commands: Commands) {

    commands.spawn(PlayerBundle {
        player: Player,
        position: Position(Vec3::ZERO),
        velocity: Velocity(Vec3::ZERO),
        sprite: Sprite {
            custom_size: Some(Vec2::new(30.0, 50.0)),
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            ..default()
        }
    });

}

fn handle_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction += Vec3::Y;
        }

        if keys.pressed(KeyCode::KeyS) {
            direction += Vec3::NEG_Y;
        }

        if keys.pressed(KeyCode::KeyA) {
            direction += Vec3::NEG_X;
        }

        if keys.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }

        velocity.0 = direction.normalize_or_zero() * 100.0;

    }

}

fn move_player(
    mut player_query: Query<(&mut Position, &Velocity), With<Player>>,
    time: Res<Time>
) {
    if let Ok((mut position, velocity)) = player_query.single_mut() {
        position.0 += velocity.0 * time.delta_secs();
    }
}

fn sync_position_transform(
    mut position_query: Query<(&Position, &mut Transform), With<Player>>
) {

    if let Ok((position, mut transform)) = position_query.single_mut() {
        transform.translation = position.0
    }

}