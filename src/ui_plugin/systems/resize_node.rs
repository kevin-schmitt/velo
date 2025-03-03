use super::{ui_helpers::ResizeMarker, RawText, RedrawArrowEvent, VeloNode, VeloNodeContainer};
use crate::{utils::convert_from_val_px, UiState};
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::CosmicEdit;
use cosmic_text::Edit;

pub fn resize_entity_start(
    mut interaction_query: Query<
        (&Interaction, &Parent, &ResizeMarker),
        (Changed<Interaction>, With<ResizeMarker>),
    >,
    mut button_query: Query<&VeloNode, With<VeloNode>>,
    mut state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, parent, resize_marker) in &mut interaction_query {
        let rectangle = button_query.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Clicked => {
                state.entity_to_resize = Some((rectangle.id, *resize_marker));
            }
            Interaction::Hovered => match *resize_marker {
                ResizeMarker::TopLeft => {
                    primary_window.cursor.icon = CursorIcon::NwseResize;
                }
                ResizeMarker::TopRight => {
                    primary_window.cursor.icon = CursorIcon::NeswResize;
                }
                ResizeMarker::BottomLeft => {
                    primary_window.cursor.icon = CursorIcon::NeswResize;
                }
                ResizeMarker::BottomRight => {
                    primary_window.cursor.icon = CursorIcon::NwseResize;
                }
            },
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub fn resize_entity_end(
    mut mouse_motion_events: EventReader<MouseMotion>,
    state: Res<UiState>,
    mut node_query: Query<
        (&VeloNodeContainer, &mut Style),
        (With<VeloNodeContainer>, Without<RawText>),
    >,
    mut raw_text_query: Query<
        (&RawText, &mut CosmicEdit),
        (Without<VeloNodeContainer>, With<RawText>),
    >,
    mut events: EventWriter<RedrawArrowEvent>,
) {
    for event in mouse_motion_events.iter() {
        if let Some((id, resize_marker)) = state.entity_to_resize {
            for (rectangle, mut button_style) in &mut node_query {
                if id == rectangle.id {
                    events.send(RedrawArrowEvent { id });
                    #[allow(unused)]
                    let mut delta = event.delta;
                    #[cfg(target_arch = "wasm32")]
                    {
                        // MouseMotion returns different values depending on platform
                        delta = Vec2::new(delta.x / 2., delta.y / 2.);
                    }
                    match resize_marker {
                        ResizeMarker::TopLeft => {
                            if let Val::Px(width) = button_style.size.width {
                                button_style.size.width = Val::Px(width - delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                button_style.size.height = Val::Px(height - delta.y);
                            }

                            if let Val::Px(x) = button_style.position.left {
                                button_style.position.left = Val::Px(x + delta.x);
                            }
                        }
                        ResizeMarker::TopRight => {
                            if let Val::Px(width) = button_style.size.width {
                                button_style.size.width = Val::Px(width + delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                button_style.size.height = Val::Px(height - delta.y);
                            }
                        }
                        ResizeMarker::BottomLeft => {
                            if let Val::Px(width) = button_style.size.width {
                                button_style.size.width = Val::Px(width - delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                button_style.size.height = Val::Px(height + delta.y);
                            }

                            if let Val::Px(x) = button_style.position.left {
                                button_style.position.left = Val::Px(x + delta.x);
                            }

                            if let Val::Px(y) = button_style.position.bottom {
                                button_style.position.bottom = Val::Px(y - delta.y);
                            }
                        }
                        ResizeMarker::BottomRight => {
                            if let Val::Px(width) = button_style.size.width {
                                button_style.size.width = Val::Px(width + delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                button_style.size.height = Val::Px(height + delta.y);
                            }

                            if let Val::Px(y) = button_style.position.bottom {
                                button_style.position.bottom = Val::Px(y - delta.y);
                            }
                        }
                    };
                    for (text, mut cosmic_edit) in &mut raw_text_query.iter_mut() {
                        if text.id == id {
                            let width = convert_from_val_px(button_style.size.width);
                            let height = convert_from_val_px(button_style.size.height);
                            cosmic_edit.size = Some((width, height));
                            cosmic_edit.editor.buffer_mut().set_redraw(true);
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{resize_entity_end, RedrawArrowEvent, VeloNodeContainer};
    use crate::{ui_plugin::ui_helpers::ResizeMarker, UiState};
    use bevy::{input::mouse::MouseMotion, prelude::*};
    use bevy_cosmic_edit::CosmicFont;

    #[test]
    fn test_resize_entity_end() {
        // Set up a test app with the necessary resources and entities
        let mut app = App::new();
        app.add_plugin(AssetPlugin::default());
        app.add_plugin(WindowPlugin::default());
        let entity_id = crate::utils::ReflectableUuid::generate();

        // Test all ResizeMarkers
        for &marker in &[
            ResizeMarker::TopLeft,
            ResizeMarker::TopRight,
            ResizeMarker::BottomLeft,
            ResizeMarker::BottomRight,
        ] {
            app.insert_resource(UiState {
                entity_to_resize: Some((entity_id, marker)),
                ..default()
            });

            app.add_event::<MouseMotion>();
            app.add_event::<RedrawArrowEvent>();
            app.add_asset::<CosmicFont>();
            app.world
                .resource_mut::<Events<MouseMotion>>()
                .send(MouseMotion {
                    delta: Vec2::new(10.0, 5.0),
                });

            app.add_system(resize_entity_end);

            app.world
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        position: UiRect {
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(VeloNodeContainer { id: entity_id });

            // Run the app
            app.update();

            // Check that the size and position of the rectangle have been updated correctly
            let (_, style) = app
                .world
                .query::<(&VeloNodeContainer, &mut Style)>()
                .iter_mut(&mut app.world)
                .last()
                .unwrap();

            match marker {
                ResizeMarker::TopLeft => {
                    assert_eq!(style.size.width, Val::Px(90.0));
                    assert_eq!(style.size.height, Val::Px(95.0));
                    assert_eq!(style.position.left, Val::Px(10.0));
                }
                ResizeMarker::TopRight => {
                    assert_eq!(style.size.width, Val::Px(120.0));
                    assert_eq!(style.size.height, Val::Px(90.0));
                }
                ResizeMarker::BottomLeft => {
                    assert_eq!(style.size.width, Val::Px(70.0));
                    assert_eq!(style.size.height, Val::Px(115.0));
                    assert_eq!(style.position.left, Val::Px(30.0));
                    assert_eq!(style.position.bottom, Val::Px(-15.0));
                }
                ResizeMarker::BottomRight => {
                    assert_eq!(style.size.width, Val::Px(140.0));
                    assert_eq!(style.size.height, Val::Px(120.0));
                    assert_eq!(style.position.bottom, Val::Px(-20.0));
                }
            }
        }
    }
}
