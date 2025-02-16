use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu);
    }
}

pub fn spawn_menu() {
    println!("this is main menu");
}
