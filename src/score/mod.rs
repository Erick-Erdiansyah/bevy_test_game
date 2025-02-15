use bevy::app::Plugin;

pub mod score_components;
pub mod score_events;
pub mod score_resources;
pub mod score_systems;

use bevy::prelude::*;
use score_events::GameOver;
use score_resources::*;
use score_systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<GameOver>()
            .init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(Update, update_score)
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            .add_systems(Update, handle_game_over);
    }
}
