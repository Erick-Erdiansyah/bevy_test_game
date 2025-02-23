use bevy::prelude::*;
use systems::{interactions::update_score_text, layouts::*};

use crate::game::{AppState, SimulationState};

pub mod components;
pub mod style;
pub mod systems;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_game_ui)
            .add_systems(
                Update,
                update_score_text
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_game_ui);
    }
}
