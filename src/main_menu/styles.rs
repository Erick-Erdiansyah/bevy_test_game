use bevy::prelude::*;

pub const NORMAL_BORDER_COLOR: Color = Color::srgb(255.0, 0.0, 0.0);
pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn main_menu_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Node::DEFAULT
    }
}

pub fn image_style() -> Node {
    Node {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        margin: UiRect {
            left: Val::Px(8.0),
            right: Val::Px(8.0),
            top: Val::Px(8.0),
            bottom: Val::Px(8.0),
        },
        ..Node::DEFAULT
    }
}

pub fn button_style() -> Node {
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        ..Node::DEFAULT
    }
}

pub fn title_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(620.0),
        height: Val::Px(120.0),
        ..Node::DEFAULT
    }
}

pub fn text_font_32(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        ..Default::default()
    }
}

pub fn text_font_64(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        ..Default::default()
    }
}
