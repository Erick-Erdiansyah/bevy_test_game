use bevy::prelude::*;

use crate::game::{score::score_resources::Score, ui::game_ui::components::ScoreText};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.0 = format!("{}", score.value.to_string());
        }
    }
}
