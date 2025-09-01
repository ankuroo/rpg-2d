use bevy::prelude::*;

use crate::components::*;
use crate::player::Player;

#[derive(Component)]
pub struct Exhausted;

pub fn update_exhaustion(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Stamina, Option<&Exhausted>), With<Player>>,
) {
    if let Ok((entity, stamina, exhausted)) = player_query.single_mut() {
        if stamina.is_depleted() && exhausted.is_none() {
            commands.entity(entity).insert(Exhausted);
        } else if stamina.is_full() && exhausted.is_some() {
            commands.entity(entity).remove::<Exhausted>();
        }
    }
}