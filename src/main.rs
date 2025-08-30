use bevy::prelude::*;

mod components;
mod states;
mod splash_screen;
mod game;
mod player;

use crate::game::GamePlugin;
use crate::player::PlayerPlugin;
use crate::states::StatePlugin;
use crate::splash_screen::SplashScreenPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(StatePlugin)
    .add_plugins(SplashScreenPlugin)
    .add_plugins(GamePlugin)
    .add_plugins(PlayerPlugin)
    .run();
}