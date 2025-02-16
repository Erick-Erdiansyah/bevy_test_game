use bevy::app::Plugin;

pub mod player_components;
pub mod player_events;
pub mod player_resources;
pub mod player_systems;

use bevy::prelude::*;
use player_systems::*;

use crate::AppState;

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    confine_player_movement.after(player_movement),
                    player_hit_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
