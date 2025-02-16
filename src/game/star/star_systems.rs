use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::star::star_components::*;
use crate::game::star::star_resources::*;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub fn spawn_stars(
    mut command: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        command.spawn((
            Transform::from_xyz(random_x, random_y, 0.0),
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut command: Commands, star_query: Query<Entity, With<Star>>) {
    for star in star_query.iter() {
        command.entity(star).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_overtime(
    mut command: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        command.spawn((
            Transform::from_xyz(random_x, random_y, 0.0),
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}
