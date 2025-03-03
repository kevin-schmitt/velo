use bevy_ui_borders::BorderColor;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::{themes::Theme, utils::ReflectableUuid};

use super::{DeleteDoc, DocListItemButton, DocListItemContainer, EditableText, GenericButton};

pub fn add_list_item(
    commands: &mut Commands,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    id: ReflectableUuid,
    name: String,
) -> Entity {
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    let root = commands
        .spawn((
            ButtonBundle {
                background_color: theme.doc_list_bg.into(),
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            GenericButton,
            DocListItemContainer { id },
            BorderColor(theme.btn_border),
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .id();
    let doc_button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.doc_list_bg.into(),
                style: Style {
                    size: Size::new(Val::Percent(90.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                ..default()
            },
            DocListItemButton { id },
            GenericButton,
        ))
        .id();
    let doc_label = commands
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
            Label,
        ))
        .id();
    let del_button = commands
        .spawn((
            ButtonBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    margin: UiRect {
                        left: Val::Px(3.),
                        right: Val::Px(3.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            DeleteDoc { id },
            GenericButton,
        ))
        .id();
    let del_label = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "\u{e14c}".to_string(),
                        style: TextStyle {
                            font_size: 24.,
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
    commands.entity(doc_button).add_child(doc_label);
    commands.entity(del_button).add_child(del_label);
    commands.entity(root).add_child(doc_button);
    commands.entity(root).add_child(del_button);
    root
}
