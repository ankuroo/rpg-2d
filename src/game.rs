use bevy::prelude::*;
use crate::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {

    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .add_systems(OnExit(GameState::InGame), cleanup_game)
        ;
    }

}

fn setup_game(mut commands: Commands) {

    commands.spawn(Camera2d);

}

fn cleanup_game(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
) {

    for entity in camera_query {
        commands.entity(entity).despawn();
    }
}