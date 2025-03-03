use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};

#[cfg(not(target_arch = "wasm32"))]
use image::*;

use std::convert::TryInto;
use uuid::Uuid;

use crate::{
    resources::{LoadTabRequest, SaveTabRequest},
    themes::Theme,
    AddRectEvent, BlinkTimer, UiState,
};

use super::ui_helpers::{get_sections, EditableText};
use crate::resources::{AppState, SaveDocRequest};

pub fn keyboard_input_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut events: EventWriter<AddRectEvent>,
    input: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut deleting: Local<bool>,
    mut editable_text_query: Query<(&mut Text, &EditableText), With<EditableText>>,
    mut blink_timer: ResMut<BlinkTimer>,
    time: Res<Time>,
    theme: Res<Theme>,
) {
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor();
    let command = input.any_pressed([KeyCode::RWin, KeyCode::LWin]);
    let shift = input.any_pressed([KeyCode::RShift, KeyCode::LShift]);
    blink_timer.timer.tick(time.delta());
    if command && input.just_pressed(KeyCode::V) {
        #[cfg(not(target_arch = "wasm32"))]
        insert_from_clipboard(&mut images, &mut events, scale_factor, &theme);
    } else if command && shift && input.just_pressed(KeyCode::S) {
        commands.insert_resource(SaveDocRequest {
            doc_id: app_state.current_document.unwrap(),
            path: None,
        });
    } else if command && input.just_pressed(KeyCode::S) {
        if let Some(current_doc) = app_state.docs.get(&app_state.current_document.unwrap()) {
            if let Some(active_tab) = current_doc.tabs.iter().find(|t| t.is_active) {
                commands.insert_resource(SaveTabRequest {
                    doc_id: app_state.current_document.unwrap(),
                    tab_id: active_tab.id,
                });
            }
        }
    } else if command && input.just_pressed(KeyCode::L) {
        if let Some(current_doc) = app_state.docs.get(&app_state.current_document.unwrap()) {
            if let Some(active_tab) = current_doc.tabs.iter().find(|t| t.is_active) {
                commands.insert_resource(LoadTabRequest {
                    doc_id: app_state.current_document.unwrap(),
                    tab_id: active_tab.id,
                    drop_last_checkpoint: true,
                });
            }
        }
    } else {
        if ui_state.doc_to_edit.is_some() || ui_state.tab_to_edit.is_some() {
            blink_timer.timer.unpause();
        } else {
            blink_timer.timer.pause();
        }
        for (mut text, editable_text) in &mut editable_text_query.iter_mut() {
            if vec![ui_state.tab_to_edit, ui_state.doc_to_edit].contains(&Some(editable_text.id))
                && input.any_just_pressed([KeyCode::Escape, KeyCode::Return])
            {
                *ui_state = UiState::default();
                commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
            }
            if Some(editable_text.id) == ui_state.doc_to_edit
                && text.sections[0].value == *"Untitled"
            {
                text.sections[0].value = "".to_string();
            }
            if vec![ui_state.tab_to_edit, ui_state.doc_to_edit].contains(&Some(editable_text.id)) {
                let mut str = "".to_string();
                let mut text_copy = text.clone();
                text_copy.sections.pop();
                for section in text_copy.sections.iter() {
                    str = format!("{}{}", str, section.value.clone());
                }
                let current_str = str.clone();
                let (str, is_del_mode) = if input.just_pressed(KeyCode::Return) {
                    (format!("{}{}", str, "\n"), false)
                } else {
                    get_text_val(str, *deleting, &input, &mut char_evr)
                };
                *deleting = is_del_mode;
                if str != current_str {
                    text.sections = get_sections(&theme, str.clone()).0;
                }
                if blink_timer.timer.finished() {
                    text.sections.last_mut().unwrap().value =
                        if text.sections.last().unwrap().value == "|" {
                            " ".to_string()
                        } else {
                            "|".to_string()
                        };
                }
                if let Some(doc_id) = ui_state.doc_to_edit {
                    let doc = app_state.docs.get_mut(&doc_id).unwrap();
                    doc.name = str.clone();
                }
                if let Some(tab_id) = ui_state.tab_to_edit {
                    if let Some(doc_id) = app_state.current_document {
                        let doc = app_state.docs.get_mut(&doc_id).unwrap();
                        if let Some(tab) = doc.tabs.iter_mut().find(|x| x.id == tab_id) {
                            tab.name = str.clone();
                        }
                    }
                }
            } else {
                text.sections.last_mut().unwrap().value = " ".to_string();
            }
        }
    }
}

fn get_text_val(
    mut str: String,
    mut deleting: bool,
    input: &Res<Input<KeyCode>>,
    char_evr: &mut EventReader<ReceivedCharacter>,
) -> (String, bool) {
    if input.just_pressed(KeyCode::Back) {
        deleting = true;
        str.pop();
    } else if input.just_released(KeyCode::Back) {
        deleting = false;
    } else {
        for ev in char_evr.iter() {
            if deleting {
                str.pop();
            } else {
                str = format!("{}{}", str, ev.char);
            }
        }
    }
    (str, deleting)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn insert_from_clipboard(
    images: &mut ResMut<Assets<Image>>,
    events: &mut EventWriter<AddRectEvent>,
    scale_factor: f64,
    theme: &Res<Theme>,
) {
    use crate::JsonNode;

    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if let Ok(image) = clipboard.get_image() {
            let image: RgbaImage = ImageBuffer::from_raw(
                image.width.try_into().unwrap(),
                image.height.try_into().unwrap(),
                image.bytes.into_owned(),
            )
            .unwrap();
            let width = image.width();
            let height = image.height();
            let size: Extent3d = Extent3d {
                width,
                height,
                ..Default::default()
            };
            let image = Image::new(
                size,
                TextureDimension::D2,
                image.to_vec(),
                TextureFormat::Rgba8UnormSrgb,
            );
            let image = images.add(image);
            events.send(AddRectEvent {
                node: JsonNode {
                    id: Uuid::new_v4(),
                    node_type: crate::NodeType::Rect,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    width: Val::Px(width as f32 / scale_factor as f32),
                    height: Val::Px(height as f32 / scale_factor as f32),
                    text: crate::JsonNodeText {
                        text: "".to_string(),
                        pos: crate::TextPos::Center,
                    },
                    bg_color: theme.clipboard_image_bg,
                    z_index: 0,
                },
                image: Some(image),
            });
        }
    }
}
