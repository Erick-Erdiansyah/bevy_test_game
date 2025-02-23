use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

use crate::game::ui::game_ui::components::*;
use crate::game::ui::game_ui::style::*;
use crate::main_menu::styles::*;

pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _game_ui_entity = build_game_ui(&mut commands, &asset_server);
}
pub fn despawn_game_ui(mut commands: Commands, game_ui_query: Query<Entity, With<GameUI>>) {
    if let Ok(game_ui_entity) = game_ui_query.get_single() {
        commands.entity(game_ui_entity).despawn_recursive();
    }
}

pub fn build_game_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let build_game_ui = commands
        .spawn((game_ui_style(), GameUI {}))
        .with_children(|parent| {
            parent
                .spawn((
                    text_ui_style(),
                    BorderColor(NORMAL_BORDER_COLOR),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/star.png"))
                            .with_mode(NodeImageMode::Auto),
                        star_image_style(),
                    ));
                    parent.spawn((
                        Text::new("0"),
                        text_font_32(&asset_server),
                        TextColor::WHITE,
                        TextLayout::new_with_justify(JustifyText::Left),
                        ScoreText {},
                    ));
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png"))
                            .with_mode(NodeImageMode::Auto),
                        star_image_style(),
                    ));
                    parent.spawn((
                        Text::new("0"),
                        text_font_32(&asset_server),
                        TextColor::WHITE,
                        TextLayout::new_with_justify(JustifyText::Left),
                        EnemyCount {},
                    ));
                });
        })
        .id();
    build_game_ui
}
