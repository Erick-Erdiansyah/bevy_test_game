use crate::{events::GameOver, game::SimulationState, AppState};
use bevy::{app::AppExit, prelude::*};

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}


pub fn handle_game_over(mut commands: Commands,mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("your final score is {}", event.score.to_string());
        commands.insert_resource(NextState::Pending(AppState::GameOver));
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            next_state.set(AppState::Game);
            println!("Game")
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("menu")
        }
    }
}
