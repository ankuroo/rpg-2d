use bevy::prelude::*;

use crate::player::action::handle_action_inputs;
use crate::player::action::InteractionEvent;
use crate::player::combat::attack_timer_system;
use crate::player::combat::handle_attack_inputs;
use crate::player::combat::AttackEvent;
use crate::states::GameState;
use crate::player::player::*;
use crate::player::movement::*;
use crate::player::state::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app

        // Events
        .add_event::<AttackEvent>()
        .add_event::<InteractionEvent>()

        // Spawns
        .add_systems(OnEnter(GameState::InGame), spawn_player)

        .add_systems(Update, (
            // Movement
            update_velocity, 
            update_position, 
            sync_position_transform, 

            // Action
            handle_action_inputs,

            // Combat
            handle_attack_inputs,
            attack_timer_system,

            // Stamina
            regen_stamina,

            // State
            update_state

        ).chain().run_if(in_state(GameState::InGame)))
        ;
    }
}