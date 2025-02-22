use bevy::prelude::*;

pub fn pause_menu_style() -> Node {
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
