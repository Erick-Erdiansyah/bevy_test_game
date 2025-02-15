pub mod enemy_components;
pub mod enemy_events;
pub mod enemy_resources;
pub mod enemy_systems;

use bevy::prelude::*;
use enemy_resources::*;
use enemy_systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, update_enemy_movement)
            .add_systems(Update, confine_enemy_movement)
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, tick_enemy_spawn_timer)
            .add_systems(Update, spawn_enemies_overtime);
    }
}
