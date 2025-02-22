use bevy::prelude::*;
use systems::{
    interactions::{interact_with_main_menu_button, interact_with_resume_button},
    layout::{despawn_pause_menu, spawn_pause_menu},
};

use crate::game::SimulationState;

pub mod components;
pub mod style;
pub mod systems;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            .add_systems(
                Update,
                (interact_with_resume_button, interact_with_main_menu_button)
                    .run_if(in_state(SimulationState::Paused)),
            )
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
