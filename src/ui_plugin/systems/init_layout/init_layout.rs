use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_cosmic_edit::{create_cosmic_font_system, CosmicFont, CosmicFontConfig};
use bevy_ui_borders::BorderColor;
use std::time::Duration;

use bevy_pkv::PkvStore;

use super::ui_helpers::{
    self, AddTab, BottomPanel, ButtonAction, LeftPanel, LeftPanelControls, LeftPanelExplorer,
    MainPanel, Menu, NewDoc, ParticlesEffect, Root, SaveDoc, TextPosMode,
};
use super::{CommChannels, ExportToFile, ImportFromFile, ImportFromUrl, ShareDoc};
use crate::canvas::arrow::components::{ArrowMode, ArrowType};
use crate::resources::{AppState, FontSystemState};
use crate::themes::Theme;
use crate::{BlinkTimer, TextPos};

#[path = "add_arrow.rs"]
mod add_arrow;
use add_arrow::*;

#[path = "add_color.rs"]
mod add_color;
use add_color::*;

#[path = "add_front_back.rs"]
mod add_front_back;
use add_front_back::*;

#[path = "add_text_pos.rs"]
mod add_text_pos;
use add_text_pos::*;

#[path = "node_manipulation.rs"]
mod node_manipulation;
use node_manipulation::*;

#[path = "add_menu_button.rs"]
mod add_menu_button;
use add_menu_button::*;

#[path = "add_list.rs"]
mod add_list;
use add_list::*;

#[path = "add_effect.rs"]
mod add_effect;
use add_effect::*;

#[path = "add_search_box.rs"]
mod add_search_box;
use add_search_box::*;

// Think about splitting this function to wasm and native
pub fn init_layout(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
    mut pkv: ResMut<PkvStore>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut fonts: ResMut<Assets<Font>>,
    theme: Res<Theme>,
) {
    // font setup
    let font_bytes_regular = include_bytes!("../../../../assets/fonts/VictorMono-Regular.ttf");
    let font_bytes_bold = include_bytes!("../../../../assets/fonts/VictorMono-Bold.ttf");
    let font_bytes_italic = include_bytes!("../../../../assets/fonts/VictorMono-Italic.ttf");
    let font_bytes_bold_italic =
        include_bytes!("../../../../assets/fonts/VictorMono-BoldItalic.ttf");
    let font_bytes_medium = include_bytes!("../../../../assets/fonts/VictorMono-Medium.ttf");
    let font_bytes_semibold = include_bytes!("../../../../assets/fonts/VictorMono-SemiBold.ttf");
    let font = Font::try_from_bytes(font_bytes_regular.to_vec()).unwrap();
    let text_style = TextStyle {
        font: TextStyle::default().font,
        font_size: 14.0,
        color: theme.font,
    };
    fonts.set_untracked(text_style.font, font);
    let cosmic_font_config = CosmicFontConfig {
        fonts_dir_path: None,
        load_system_fonts: true,
        font_bytes: Some(vec![
            font_bytes_regular,
            font_bytes_italic,
            font_bytes_bold,
            font_bytes_bold_italic,
            font_bytes_medium,
            font_bytes_semibold,
        ]),
    };
    let font_system = create_cosmic_font_system(cosmic_font_config);
    let cosmic_font_handle = cosmic_fonts.add(CosmicFont(font_system));
    commands.insert_resource(FontSystemState(Some(cosmic_font_handle.clone())));

    let primary_window: &Window = windows.single();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let (tx, rx) = async_channel::bounded(1);
        commands.insert_resource(CommChannels { tx, rx });
    }
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    commands.insert_resource(BlinkTimer {
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    });
    let bottom_panel = commands
        .spawn((
            NodeBundle {
                background_color: theme.bottom_panel_bg.into(),
                style: Style {
                    border: UiRect::all(Val::Px(1.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Percent(0.),
                        right: Val::Percent(0.),
                        bottom: Val::Percent(0.),
                        top: Val::Percent(96.),
                    },
                    size: Size::new(Val::Percent(100.), Val::Percent(4.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            BottomPanel,
            BorderColor(theme.btn_border),
        ))
        .id();
    let add_tab = add_menu_button(
        &mut commands,
        &theme,
        "New Tab".to_string(),
        &icon_font,
        AddTab,
    );
    commands.entity(bottom_panel).add_child(add_tab);

    let docs = add_list(&mut commands, &theme, &mut app_state, &mut pkv);

    let root_ui = commands
        .spawn((
            NodeBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Root,
        ))
        .id();

    let menu = commands
        .spawn((
            NodeBundle {
                background_color: theme.menu_bg.into(),
                style: Style {
                    border: UiRect::all(Val::Px(1.0)),
                    size: Size::new(Val::Percent(100.0), Val::Percent(5.)),
                    padding: UiRect {
                        left: Val::Px(10.),
                        ..default()
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
            BorderColor(theme.btn_border),
            Menu,
        ))
        .id();
    let new_doc = add_menu_button(
        &mut commands,
        &theme,
        "New Document".to_string(),
        &icon_font,
        NewDoc,
    );
    let save_doc = add_menu_button(
        &mut commands,
        &theme,
        "Save Document".to_string(),
        &icon_font,
        SaveDoc,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let export_file = add_menu_button(
        &mut commands,
        &theme,
        "Export To File".to_string(),
        &icon_font,
        ExportToFile,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let import_file = add_menu_button(
        &mut commands,
        &theme,
        "Import From File".to_string(),
        &icon_font,
        ImportFromFile,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let import_url = add_menu_button(
        &mut commands,
        &theme,
        "Import From URL".to_string(),
        &icon_font,
        ImportFromUrl,
    );
    #[cfg(target_arch = "wasm32")]
    let set_window_prop = add_menu_button(
        &mut commands,
        &theme,
        "Save Document to window.velo object".to_string(),
        &icon_font,
        super::SetWindowProperty,
    );
    commands.entity(menu).add_child(new_doc);
    commands.entity(menu).add_child(save_doc);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(menu).add_child(export_file);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(menu).add_child(import_file);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(menu).add_child(import_url);
    if app_state.github_token.is_some() {
        let share_doc = add_menu_button(
            &mut commands,
            &theme,
            "Share Document (copy URL to clipboard)".to_string(),
            &icon_font,
            ShareDoc,
        );
        commands.entity(menu).add_child(share_doc);
    }
    #[cfg(target_arch = "wasm32")]
    commands.entity(menu).add_child(set_window_prop);

    let main_bottom = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(95.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .id();
    let left_panel = commands
        .spawn((
            NodeBundle {
                background_color: theme.left_panel_bg.into(),
                style: Style {
                    border: UiRect::all(Val::Px(1.0)),
                    size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(theme.btn_border),
            LeftPanel,
        ))
        .id();
    let right_panel = commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(85.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .id();
    let main_panel = commands
        .spawn((
            ButtonBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            MainPanel,
        ))
        .id();

    commands.entity(right_panel).add_child(main_panel);
    commands.entity(right_panel).add_child(bottom_panel);

    let left_panel_controls = commands
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    size: Size::new(Val::Percent(100.), Val::Percent(40.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanelControls,
        ))
        .id();
    #[cfg(not(target_arch = "wasm32"))]
    let search_box = add_search_box(
        &mut commands,
        &theme,
        &mut cosmic_fonts,
        cosmic_font_handle,
        primary_window.scale_factor() as f32,
    );
    let left_panel_explorer = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(60.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanelExplorer,
        ))
        .id();
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(left_panel_explorer).add_child(search_box);
    commands.entity(left_panel_explorer).add_child(docs);

    commands.entity(left_panel).add_child(left_panel_controls);
    commands.entity(left_panel).add_child(left_panel_explorer);

    let rectangle_creation = node_manipulation(
        &mut commands,
        &theme,
        &icon_font,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::AddRec,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::AddCircle,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Del,
        },
    );
    let fron_back = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.), Val::Percent(10.)),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let front = add_front_back(
        &mut commands,
        &theme,
        &asset_server,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Front,
        },
    );
    let back = add_front_back(
        &mut commands,
        &theme,
        &asset_server,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Back,
        },
    );
    commands.entity(fron_back).add_child(front);
    commands.entity(fron_back).add_child(back);

    let color_picker = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.), Val::Percent(9.)),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let color1 = add_color(
        &mut commands,
        &theme,
        Color::rgb(1., 225.0 / 255.0, 130.0 / 255.0),
    );
    let color2 = add_color(
        &mut commands,
        &theme,
        Color::rgb(215.0 / 255.0, 204.0 / 255.0, 200.0 / 255.0),
    );
    let color3 = add_color(
        &mut commands,
        &theme,
        Color::rgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0),
    );
    let color4 = add_color(
        &mut commands,
        &theme,
        Color::rgb(207.0 / 255.0, 226.0 / 255.0, 243.0 / 255.0),
    );
    let color5 = add_color(
        &mut commands,
        &theme,
        Color::rgb(245.0 / 255.0, 222.0 / 255.0, 179.0 / 255.0),
    );

    commands.entity(color_picker).add_child(color1);
    commands.entity(color_picker).add_child(color2);
    commands.entity(color_picker).add_child(color3);
    commands.entity(color_picker).add_child(color4);
    commands.entity(color_picker).add_child(color5);

    let arrow_modes = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.), Val::Percent(9.)),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let arrow1 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Line,
        },
    );
    let arrow2 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Arrow,
        },
    );
    let arrow3 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::DoubleArrow,
        },
    );
    let arrow4 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelLine,
        },
    );
    let arrow5 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelArrow,
        },
    );
    let arrow6 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelDoubleArrow,
        },
    );
    commands.entity(arrow_modes).add_child(arrow1);
    commands.entity(arrow_modes).add_child(arrow2);
    commands.entity(arrow_modes).add_child(arrow3);
    commands.entity(arrow_modes).add_child(arrow4);
    commands.entity(arrow_modes).add_child(arrow5);
    commands.entity(arrow_modes).add_child(arrow6);

    let text_modes = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.), Val::Percent(10.)),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let text_pos1 = add_text_pos(
        &mut commands,
        &theme,
        &asset_server,
        TextPosMode {
            text_pos: TextPos::Center,
        },
    );
    let text_pos2 = add_text_pos(
        &mut commands,
        &theme,
        &asset_server,
        TextPosMode {
            text_pos: TextPos::TopLeft,
        },
    );
    commands.entity(text_modes).add_child(text_pos1);
    commands.entity(text_modes).add_child(text_pos2);

    #[cfg(not(target_arch = "wasm32"))]
    let effects = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.), Val::Percent(10.)),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let effect1 = add_effect(&mut commands, &theme, &icon_font, ParticlesEffect);
        commands.entity(effects).add_child(effect1);
    }

    commands
        .entity(left_panel_controls)
        .add_child(rectangle_creation);
    commands.entity(left_panel_controls).add_child(color_picker);
    commands.entity(left_panel_controls).add_child(arrow_modes);
    commands.entity(left_panel_controls).add_child(text_modes);
    commands.entity(left_panel_controls).add_child(fron_back);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(left_panel_controls).add_child(effects);

    commands.entity(main_bottom).add_child(left_panel);
    commands.entity(main_bottom).add_child(right_panel);
    commands.entity(root_ui).add_child(menu);
    commands.entity(root_ui).add_child(main_bottom);
}
