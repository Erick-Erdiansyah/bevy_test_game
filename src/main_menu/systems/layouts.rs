use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::*,
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}
pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((main_menu_style(), MainMenu {}))
        .with_children(|parent| {
            // ======= ALSO NOT PLAY BUTTON =======
            parent.spawn(title_style()).with_children(|parent| {
                // Image
                parent.spawn((
                    ImageNode::new(asset_server.load("sprites/ball_blue_large.png"))
                        .with_mode(NodeImageMode::Auto),
                    image_style(),
                ));
                //  Text
                parent.spawn((
                    Text::new("Bevy Ball Game"),
                    text_font_64(&asset_server),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
                // Image
                parent.spawn((
                    ImageNode::new(asset_server.load("sprites/ball_red_large.png"))
                        .with_mode(NodeImageMode::Auto),
                    image_style(),
                ));
            });
            // ======= PLAY BUTTON =======
            parent
                .spawn((
                    Button,
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        text_font_32(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // ======= NOT PLAY BUTTON =======
            parent
                .spawn((
                    Button,
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        text_font_32(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        })
        .id();
    main_menu_entity
}