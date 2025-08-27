use bevy::prelude::*;

#[derive(Component)]
pub struct SplashScreen;

#[derive(Component)]
pub enum SplashState {
    Initial(Timer),
    FadeIn(Timer),
    Wait(Timer),
    FadeOut(Timer),
}

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {

    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_splash)
        .add_systems(Update, fade_splash)
        ;
    }

}

fn ease_fade(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t) // smoothstep
}


fn fade_splash(
    time: Res<Time>,
    mut splash_query: Query<(&mut SplashState, &mut ImageNode), With<SplashScreen>>
) {

    if let Ok((mut state, mut image)) = splash_query.single_mut() {

        match &mut *state {

            SplashState::Initial(x) => {
                x.tick(time.delta());
                if x.finished() {
                    *state = SplashState::FadeIn(Timer::from_seconds(2.0, TimerMode::Once));
                }
            },

            SplashState::FadeIn(x) => {
                x.tick(time.delta());
                image.color.set_alpha(ease_fade(x.fraction()));
                if x.finished() {
                    *state = SplashState::Wait(Timer::from_seconds(3.0, TimerMode::Once));
                }
            },

            SplashState::Wait(x) => {
                x.tick(time.delta());
                if x.finished() {
                    *state = SplashState::FadeOut(Timer::from_seconds(2.0, TimerMode::Once));
                }
            },

            SplashState::FadeOut(x) => {
                x.tick(time.delta());
                image.color.set_alpha(ease_fade(1.0 - x.fraction()));
            },

        }
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
            SplashScreen,
            SplashState::Initial(Timer::from_seconds(2.0, TimerMode::Once)),
            ImageNode {
                image: asset_server.load("splash.png"),
                color: Color::linear_rgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            Node {
                ..Default::default()
            },
        ));
    });

}