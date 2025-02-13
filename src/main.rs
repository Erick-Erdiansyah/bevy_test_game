use bevy::prelude::*;
pub mod components;
pub mod resources;
pub mod events;
mod systems;

use events::*;
use resources::*;
use systems::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScore>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_palyer)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_movement)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_overtime)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemies_overtime)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, exit_game)
        .run();
}

