use bevy::{prelude::*, window::PrimaryWindow};

use crate::{score::score_resources::Score, star::star_components::Star, star::star_systems::STAR_SIZE};

use super::player_components::Player;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;


pub fn spawn_palyer(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = window_query.get_single().unwrap();
  commands.spawn((
      Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      Sprite {
          image: asset_server.load("sprites/ball_blue_large.png"),
          ..Default::default()
      },
      Player {},
  ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
  let window = window_query.get_single().unwrap();
  commands.spawn((
      Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      Camera2d {
          ..Default::default()
      },
  ));
}


pub fn player_movement(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut player_query: Query<&mut Transform, With<Player>>,
  time: Res<Time>,
) {
  if let Ok(mut transform) = player_query.get_single_mut() {
      let mut direction = Vec3::ZERO;

      if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
          direction += Vec3::new(-1.0, 0.0, 0.0);
      }
      if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
          direction += Vec3::new(1.0, 0.0, 0.0);
      }
      if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
          direction += Vec3::new(0.0, 1.0, 0.0);
      }
      if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
          direction += Vec3::new(0.0, -1.0, 0.0);
      }

      if direction.length() > 0.0 {
          direction = direction.normalize();
      }

      transform.translation += direction * PLAYER_SPEED * time.delta_secs();
  }
}

pub fn confine_player_movement(
  mut player_query: Query<&mut Transform, With<Player>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(mut player_transform) = player_query.get_single_mut() {
      let window = window_query.get_single().unwrap();

      let half_player_size = PLAYER_SIZE / 2.0;
      let x_min = 10.0 + half_player_size;
      let x_max = window.width() - half_player_size;
      let y_min = 10.0 + half_player_size;
      let y_max = window.height() - half_player_size;

      let mut translation = player_transform.translation;

      if translation.x < x_min {
          translation.x = x_min
      } else if translation.x > x_max {
          translation.x = x_max
      }

      if translation.y < y_min {
          translation.y = y_min
      } else if translation.y > y_max {
          translation.y = y_max
      }

      player_transform.translation = translation;
  }
}

pub fn player_hit_star(
  mut command: Commands,
  star_query: Query<(Entity, &Transform), With<Star>>,
  player_query: Query<&Transform, With<Player>>,
  mut score: ResMut<Score>,
) {
  if let Ok(player_transform) = player_query.get_single() {
      for (star_entity, star_transform) in star_query.iter() {
          let distance = star_transform
              .translation
              .distance(player_transform.translation);
          let player_radius = PLAYER_SIZE / 2.0;
          let star_radius = STAR_SIZE / 2.0;
          if distance < player_radius + star_radius {
              println!("Get star !!!");
              score.value += 1;
              command.entity(star_entity).despawn();
          }
      }
  }
}