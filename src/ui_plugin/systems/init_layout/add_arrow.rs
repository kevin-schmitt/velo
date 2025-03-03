use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{GenericButton, TooltipPosition},
};

use super::ui_helpers::{get_tooltip, Tooltip};
use crate::canvas::arrow::components::{ArrowMode, ArrowType};
pub fn add_arrow(
    commands: &mut Commands,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    arrow_mode: ArrowMode,
) -> Entity {
    let (image, text) = match arrow_mode.arrow_type {
        ArrowType::Line => (asset_server.load("line.png"), "Enable line mode"),
        ArrowType::Arrow => (asset_server.load("arrow.png"), "Enable single arrow mode"),
        ArrowType::DoubleArrow => (
            asset_server.load("double-arrow.png"),
            "Enable double arrow mode",
        ),
        ArrowType::ParallelLine => (
            asset_server.load("parallel-line.png"),
            "Enable parallel line mode",
        ),
        ArrowType::ParallelArrow => (
            asset_server.load("parallel-arrow.png"),
            "Enable parallel arrow mode",
        ),
        ArrowType::ParallelDoubleArrow => (
            asset_server.load("parallel-double-arrow.png"),
            "Enable parallel double arrow mode",
        ),
    };
    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(3.)),
                size: Size::new(Val::Percent(13.), Val::Percent(100.)),
                ..default()
            },
            background_color: theme.shadow.into(),
            ..default()
        })
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.arrow_btn_bg.into(),
                image: image.into(),
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(-2.),
                        right: Val::Px(0.),
                        top: Val::Px(-2.),
                        bottom: Val::Px(0.),
                    },
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(theme.btn_border),
            arrow_mode,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip(theme, text.to_string(), 14., TooltipPosition::Bottom),
                Tooltip,
            ));
        })
        .id();
    commands.entity(top).add_child(button);
    top
}
