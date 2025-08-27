use bevy::prelude::*;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {

    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_splash)
        ;
    }

}

fn setup_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
    ))
    .with_children(|parent| {
        parent.spawn((
            ImageNode {
                image: asset_server.load("splash.png"),
                ..default()
            },
            Node {
                ..Default::default()
            },
        ));
    });

}