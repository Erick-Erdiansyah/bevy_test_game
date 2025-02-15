pub mod star_systems;
pub mod star_events;
pub mod star_resources;
pub mod star_components;

use bevy::prelude::*;
use star_systems::*;
use star_resources::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_overtime);
    }
}