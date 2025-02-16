use bevy::app::Plugin;

pub mod score_components;
pub mod score_resources;
pub mod score_systems;

use bevy::prelude::*;
use score_resources::*;
use score_systems::*;

use crate::AppState;

use super::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), high_scores_updated)
            .add_systems(OnExit(AppState::Game), remove_resource)
            .add_systems(
                Update,
                (update_score, update_high_scores)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
