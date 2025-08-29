use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component)]
pub struct MainMenuCamera;

#[derive(Component)]
pub struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {

    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::MainMenu),setup_menu)
        .add_systems(OnExit(GameState::MainMenu),cleanup_menu)
        ;
    }

}

fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenu>>,
    camera_query: Query<Entity, With<MainMenuCamera>>,
) {

    if let Ok(entity) = menu_query.single() {
        commands.entity(entity).despawn();
    }

    if let Ok(entity) = camera_query.single() {
        commands.entity(entity).despawn();
    }

}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn((
        MainMenuCamera,
        Camera2d,
    ));

    commands.spawn((
        MainMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
    )).with_children(|parent| {

        parent.spawn((
            ImageNode {
                image: asset_server.load("logo.png"),
                ..default()
            },
            Node {..Default::default()}
        ));

    }
    );
}
