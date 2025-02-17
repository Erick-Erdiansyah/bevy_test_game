use bevy::prelude::*;
use systems::layouts::*;

use crate::AppState;

pub mod components;
pub mod styles;
pub mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
