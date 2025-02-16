pub mod star_components;
pub mod star_resources;
pub mod star_systems;

use bevy::prelude::*;
use star_resources::*;
use star_systems::*;

use crate::AppState;

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_overtime)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
