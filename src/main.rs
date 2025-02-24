use bevy::prelude::*;
mod systems;
mod events;
mod game;
mod main_menu;

use systems::*;
use events::GameOver;
use game::GamePlugin;
use main_menu::MenuPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .init_state::<AppState>()
    .add_plugins(MenuPlugin)
    .add_plugins(GamePlugin)
    .add_event::<GameOver>()
    .add_systems(Update, handle_game_over)
    .add_systems(Update, transition_to_game_state)
    .add_systems(Update, transition_to_main_menu_state)
    .add_systems(Update, exit_game)
        .run();
}

#[derive(States,Debug,Clone,Copy,PartialEq, Eq,Hash,Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}