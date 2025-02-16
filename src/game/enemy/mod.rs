pub mod enemy_components;
pub mod enemy_events;
pub mod enemy_resources;
pub mod enemy_systems;

use bevy::prelude::*;
use enemy_resources::*;
use enemy_systems::*;

use crate::AppState;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement.after(spawn_enemies),
                    update_enemy_movement.after(enemy_movement),
                    confine_enemy_movement.after(update_enemy_movement),
                    enemy_hit_player,
                    tick_enemy_spawn_timer,
                    spawn_enemies_overtime,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
