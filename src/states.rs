use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    InGame
}

pub struct StatePlugin;

impl Plugin for StatePlugin {

    fn build(&self, app: &mut App) {
        app
        .insert_state(GameState::Splash)
        ;
    }

}