use bevy::app::Plugin;

pub mod player_systems;
pub mod player_events;
pub mod player_resources;
pub mod player_components;

use bevy::prelude::*;
use player_systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
      app
      .add_systems(Startup, spawn_camera)
      .add_systems(Startup, spawn_player)
      .add_systems(Update, player_movement)
      .add_systems(Update, confine_player_movement.after(player_movement))
      .add_systems(Update, player_hit_star);
    }
}