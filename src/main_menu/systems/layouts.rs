use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::NORMAL_BUTTON_COLOR,
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}
pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // ======= ALSO NOT PLAY BUTTON =======
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(620.0),
                    height: Val::Px(120.0),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png"))
                            .with_mode(NodeImageMode::Auto),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect {
                                left: Val::Px(8.0),
                                right: Val::Px(8.0),
                                top: Val::Px(8.0),
                                bottom: Val::Px(8.0),
                            },
                            ..Default::default()
                        },
                    ));
                    //  Text
                    parent.spawn((
                        Text::new("Bevy Ball Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    // Image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png"))
                            .with_mode(NodeImageMode::Auto),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect {
                                left: Val::Px(8.0),
                                right: Val::Px(8.0),
                                top: Val::Px(8.0),
                                bottom: Val::Px(8.0),
                            },
                            ..Default::default()
                        },
                    ));
                });
            // ======= PLAY BUTTON =======
            parent
                .spawn((
                    Button,
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..Default::default()
                    },
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // ======= NOT PLAY BUTTON =======
            parent
                .spawn((
                    Button,
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..Default::default()
                    },
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        })
        .id();
    main_menu_entity
}
