use bevy::prelude::*;

mod states;
mod splash_screen;
mod main_menu;

use crate::main_menu::MainMenuPlugin;
use crate::states::StatePlugin;
use crate::splash_screen::SplashScreenPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(StatePlugin)
    .add_plugins(SplashScreenPlugin)
    .add_plugins(MainMenuPlugin)
    .run();
}
