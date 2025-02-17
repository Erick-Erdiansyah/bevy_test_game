use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use crate::AppState;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
        .add_systems(OnEnter(AppState::Game), pause_simulation)
        .add_systems(OnExit(AppState::Game), resume_simulation)
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
