use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::CosmicFont;

use crate::{resources::FontSystemState, themes::Theme, utils::ReflectableUuid};

use super::{spawn_node, AddRectEvent, MainPanel, NodeMeta, UiState};

pub fn create_new_node(
    mut commands: Commands,
    mut events: EventReader<AddRectEvent>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {
    let window = windows.single_mut();
    for event in events.iter() {
        *ui_state = UiState::default();
        ui_state.entity_to_edit = Some(ReflectableUuid(event.node.id));
        let entity = spawn_node(
            &mut commands,
            &theme,
            &asset_server,
            &mut cosmic_fonts,
            font_system_state.0.clone().unwrap(),
            window.scale_factor() as f32,
            NodeMeta {
                size: (event.node.width, event.node.height),
                id: ReflectableUuid(event.node.id),
                node_type: event.node.node_type.clone(),
                image: event.image.clone(),
                text: event.node.text.text.clone(),
                bg_color: event.node.bg_color,
                position: (event.node.left, event.node.bottom),
                text_pos: event.node.text.pos.clone(),
                z_index: event.node.z_index,
                is_active: true,
            },
        );
        commands.entity(main_panel_query.single()).add_child(entity);
    }
}
