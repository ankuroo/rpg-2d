use bevy::prelude::*;

mod states;
mod splash_screen;

use crate::states::StatePlugin;
use crate::splash_screen::SplashScreenPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(StatePlugin)
    .add_plugins(SplashScreenPlugin)
    .run();
}