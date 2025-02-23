use bevy::prelude::*;

mod pause_menu;
mod game_ui;
use game_ui::GameUI;
use pause_menu::PausePlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PausePlugin).add_plugins(GameUI);
    }
}
