use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::player::player_components::*, game::score::score_resources::Score,
    game::star::star_components::Star, game::star::star_systems::STAR_SIZE,
};

use super::player_components::Player;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();

    let texture = asset_server.load("sprites/ball_Sheet.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config = AnimationConfig::new(0, 3, 12);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            flip_x: true,
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        )),
        Player {},
        animation_config,
    ));
}

pub fn execute_animation(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    // i think the frame should move when the direction change
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

pub fn despawn_player(mut command: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        command.entity(player).despawn();
    }
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
    mut player_query: Query<(&mut Transform, &mut AnimationConfig), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut animation)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
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
