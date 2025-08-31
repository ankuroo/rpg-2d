use bevy::prelude::*;

mod components;
mod player;
mod states;
mod splash_screen;

use crate::player::PlayerPlugin;
use crate::states::{GameState, StatePlugin};
use crate::splash_screen::SplashScreenPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(StatePlugin)
    .add_plugins(SplashScreenPlugin)
    .add_plugins(PlayerPlugin)
    .add_systems(OnEnter(GameState::InGame), spawn_test_camera)
    .run();
}

fn spawn_test_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}