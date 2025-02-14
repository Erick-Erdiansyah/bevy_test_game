use bevy::prelude::*;

use crate::score::score_resources::*;
use crate::score::score_events::*;

pub fn update_score(score: Res<Score>) {
  if score.is_changed() {
      println!("Score : {}", score.value.to_string());
  }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
  for event in game_over_event_reader.read() {
      println!("your final score is {}", event.score.to_string());
  }
}

pub fn update_high_scores(
  mut game_over_event_reader: EventReader<GameOver>,
  mut high_scores: ResMut<HighScore>,
) {
  for event in game_over_event_reader.read() {
      high_scores.scores.push(("Player".to_string(), event.score));
  }
}

pub fn high_scores_updated(high_scores: Res<HighScore>) {
  if high_scores.is_changed() {
      println!("High scores: {:?}", high_scores);
  }
}
