use bevy::prelude::*;

use crate::{
    game::ui::pause_menu::{components::*, style::pause_menu_style},
    main_menu::{components::MainMenu, styles::*},
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _pause_menu_entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((pause_menu_style(), PauseMenu {}))
        .with_children(|parent| {
            parent.spawn(title_style()).with_children(|parent| {
                parent.spawn((
                    Text::new("PAUSED"),
                    text_font_64(&asset_server),
                    TextColor::WHITE,
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
            });
            parent
                .spawn((
                    Button,
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    MainMenu {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("MAIN MENU"),
                        text_font_32(&asset_server),
                        TextColor::WHITE,
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            parent
                .spawn((
                    Button,
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    ResumeButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PLAY"),
                        text_font_32(&asset_server),
                        TextColor::WHITE,
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        })
        .id();
    pause_menu_entity
}
