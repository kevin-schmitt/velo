use bevy::prelude::*;

use crate::{themes::Theme, utils::ReflectableUuid};

use super::{DeleteTab, EditableText, GenericButton, TabButton, TabContainer};

pub fn add_tab(
    commands: &mut Commands,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    name: String,
    id: ReflectableUuid,
    is_active: bool,
) -> Entity {
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    let root = commands
        .spawn((
            NodeBundle {
                background_color: theme.add_tab_bg.into(),
                style: Style {
                    size: Size::new(Val::Percent(8.), Val::Percent(90.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                ..default()
            },
            TabContainer { id },
        ))
        .id();
    let tab_button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.add_tab_bg.into(),
                style: Style {
                    size: Size::new(Val::Percent(90.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            GenericButton,
            TabButton { id },
        ))
        .id();
    let tab_label = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: name,
                            style: TextStyle {
                                font_size: 18.,
                                color: theme.font,
                                ..default()
                            },
                        },
                        TextSection {
                            value: " ".to_string(),
                            style: TextStyle {
                                font_size: 18.,
                                color: theme.font,
                                ..default()
                            },
                        },
                    ],
                    ..default()
                },
                style: Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            EditableText { id },
        ))
        .id();
    let del_button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.add_tab_bg.into(),
                visibility: if is_active {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                style: Style {
                    margin: UiRect {
                        left: Val::Px(3.),
                        right: Val::Px(3.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GenericButton,
            DeleteTab { id },
        ))
        .id();
    let del_label = commands
        .spawn((
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection {
                        value: "\u{e14c}".to_string(),
                        style: TextStyle {
                            font_size: 18.,
                            color: theme.del_button,
                            font: icon_font,
                        },
                    }],
                    ..default()
                },
                ..default()
            },
            Label,
        ))
        .id();
    commands.entity(tab_button).add_child(tab_label);
    commands.entity(del_button).add_child(del_label);
    commands.entity(root).add_child(tab_button);
    commands.entity(root).add_child(del_button);
    root
}
