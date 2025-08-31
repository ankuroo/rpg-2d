use bevy::prelude::*;

#[derive(Event)]
pub struct InteractionEvent;

pub fn handle_action_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut interaction_writer: EventWriter<InteractionEvent>,
) {
    
    if keys.just_pressed(KeyCode::KeyE) {
        interaction_writer.write(InteractionEvent);
    }
}