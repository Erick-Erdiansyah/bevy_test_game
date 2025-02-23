use bevy::prelude::*;

use crate::game::{
    enemy::enemy_components::Enemy,
    score::score_resources::Score,
    ui::game_ui::components::{EnemyCount, ScoreText},
};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.0 = format!("{}", score.value.to_string());
        }
    }
}

pub fn update_enemy_ball(
    mut text_query: Query<&mut Text, With<EnemyCount>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.0 = format!("{}", count.to_string());
    }
}
