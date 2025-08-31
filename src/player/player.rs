use std::fs;

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::components::*;
use crate::player::PlayerState;

#[derive(Component)]
pub struct Player;

#[derive (Serialize, Deserialize)]
pub struct PlayerConfig {
    pub health : f32,
    pub stamina: f32,
    pub start_position: [f32; 3],
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

fn load_player_config(path: &str) -> PlayerConfig {
    let file_contents = fs::read_to_string(path)
        .expect("Failed to read config file");

    serde_json::from_str(&file_contents)
        .expect("Failed to parse JSON")

}

pub fn regen_stamina(
    time: Res<Time>,
    mut player_query: Query<(&PlayerState, &mut Stamina), With<Player>>
) {
    if let Ok((state, mut stamina)) = player_query.single_mut() {
        if !matches!(*state, PlayerState::Dead | PlayerState::Attacking) {
            stamina.regen(5.0 * time.delta_secs());
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    let config = load_player_config("assets/configs/player.json");
    commands.spawn(PlayerBundle::from_config(config));
}