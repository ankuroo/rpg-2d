use std::fs;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::*;

#[derive(Component)]
pub struct Player;

#[derive (Serialize, Deserialize)]
pub struct PlayerConfig {
    pub health : f32,
    pub stamina: f32,
    pub start_position: [f32; 3],
}

#[derive(Component, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
    Exhausted,
    Dead
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: Position,
    velocity: Velocity,
    sprite: Sprite,
    health: Health,
    stamina: Stamina,
    state: PlayerState,
}

impl PlayerBundle {

    pub fn default() -> Self {
        Self {
            player: Player,
            position: Position(Vec3::ZERO),
            velocity: Velocity(Vec3::ZERO),
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 50.0)),
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..default()
            },
            health: Health::new(30.0),
            stamina: Stamina::new( 15.0),
            state: PlayerState::Idle,
        }
    }

    pub fn from_config(config: PlayerConfig) -> Self {
        Self {
            player: Player,
            position: Position(Vec3::from(config.start_position)),
            velocity: Velocity(Vec3::ZERO),
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 50.0)),
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..default()
            },
            health: Health::new(config.health),
            stamina: Stamina::new(config.stamina),
            state: PlayerState::Idle,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<AttackEvent>()
        .add_event::<InteractionEvent>()
        .add_systems(Startup, spawn_player_from_config)
        .add_systems(Update, (
            handle_movement_inputs, 
            handle_action_inputs, 
            attack_timer_system,
            move_player, 
            manage_health, 
            update_player_state,
            stamina_regen,
            sync_position_transform
        ).chain())
        ;
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn load_player_config(path: &str) -> PlayerConfig {
    let file_contents = fs::read_to_string(path)
        .expect("Failed to read config file");

    serde_json::from_str(&file_contents)
        .expect("Failed to parse JSON")

}

fn spawn_player_from_config(mut commands: Commands) {
    let config = load_player_config("assets/configs/player.json");
    commands.spawn(PlayerBundle::from_config(config));
}

fn handle_movement_inputs(
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

#[derive(Event)]
pub struct AttackEvent;

#[derive(Component)]
pub struct AttackTimer(Timer);

#[derive(Event)]
pub struct InteractionEvent;

fn handle_action_inputs(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Stamina, &mut PlayerState), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut attack_event: EventWriter<AttackEvent>,
    mut interaction_event: EventWriter<InteractionEvent>,
) {

    // Attacks
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok((entity, mut stamina, mut state)) = player_query.single_mut() {
            if stamina.current >= 5.0 && *state != PlayerState::Exhausted && *state != PlayerState::Attacking {
                stamina.deplete(5.0);
                attack_event.write(AttackEvent);
                *state = PlayerState::Attacking;
                commands.entity(entity).insert(AttackTimer(Timer::from_seconds(0.3, TimerMode::Once)));
            }
        }
    }

    // Actions
    if keys.just_pressed(KeyCode::KeyE) {
        interaction_event.write(InteractionEvent);
    }
}

fn attack_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(Entity, &mut AttackTimer, &mut PlayerState), With<Player>>
) {
    for (entity, mut timer, mut state) in player_query {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            *state = PlayerState::Idle; 
            commands.entity(entity).remove::<AttackTimer>();
        }
    }
}

fn manage_health(
    mut health_query: Query<(Entity, &mut Health), With<Player>>,
    mut commands: Commands
) {

    if let Ok((entity, mut health)) = health_query.single_mut() {
        if !health.is_alive() {
            println!("Player is dead.");
            commands.entity(entity).despawn();
        }
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

fn stamina_regen(
    time: Res<Time>,
    mut stamina_query: Query<(&mut Stamina, &PlayerState), With<Player>>
) {
    if let Ok((mut stamina, state)) = stamina_query.single_mut() {
        if !matches!(*state, PlayerState::Attacking | PlayerState::Dead) { stamina.regen(5.0 * time.delta_secs()); }
    }

}

fn sync_position_transform(
    mut position_query: Query<(&Position, &mut Transform), With<Player>>
) {

    if let Ok((position, mut transform)) = position_query.single_mut() {
        transform.translation = position.0
    }

}

fn update_player_state(
    mut player_query: Query<(&mut PlayerState, &Health, &mut Stamina, &Velocity), With<Player>>,
) {

    if let Ok((mut state, health, mut stamina, velocity)) = player_query.single_mut() {

        if !health.is_alive() { *state = PlayerState::Dead; }

        else if (*state == PlayerState::Attacking) {}

        else if stamina.is_depleted() { *state = PlayerState::Exhausted; }

        else if (*state == PlayerState::Exhausted && stamina.is_full()) {

            if velocity.0.length() == 0.0 { *state = PlayerState::Idle; } 
            else { *state = PlayerState::Walking; }

        } else if *state != PlayerState::Exhausted  {

            if velocity.0.length() == 0.0 { *state = PlayerState::Idle; } 
            else { *state = PlayerState::Walking; }

        }
    }
}