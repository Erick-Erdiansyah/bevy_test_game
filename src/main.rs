use bevy::prelude::*;
mod enemy;
mod player;
mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::exit_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)        
        .add_plugins(StarPlugin)
        .add_systems(Update, exit_game)
        .run();
}

