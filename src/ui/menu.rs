use bevy::prelude::*;

use crate::{
    res::{Clear, GameState, UISelectInfo, MENU_LIST},
    utils::{
        class::StyleCommand,
        widget::{atlas_image, grid, image, node_children, node_root, node_text, GridItemInfo},
    },
};

use super::class::menu_class::{
    class_node_game_logo_image, class_node_game_panel, class_node_menu, class_node_menu_item,
    class_node_menu_text, class_node_menu_text_default, class_node_menu_text_hover,
    class_node_root, class_node_tank_selector,
};

pub fn setup_ui_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    node_root(class_node_root, commands, Clear, |gc| {
        node_children(class_node_game_panel, gc, (), |gc| {
            image(class_node_game_logo_image, gc, ());
            grid(
                3,
                2,
                48.,
                class_node_menu,
                class_node_menu_item,
                gc,
                (),
                |gc, r, c| match c {
                    1 => {
                        node_text(MENU_LIST[r], class_node_menu_text, gc, Interaction::None);
                    }
                    _ => {
                        atlas_image(class_node_tank_selector, gc, GridItemInfo(r, c));
                    }
                },
            );
        });
    });
}

pub fn update_ui_menu(
    mut commands: Commands,
    query_hover: Query<&Interaction>,
    query_entity: Query<&Parent, With<Interaction>>,
    key_event: Res<ButtonInput<KeyCode>>,
    mut query_visible: Query<&mut Visibility, With<TextureAtlas>>,
    mut ui_select_info: ResMut<UISelectInfo>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key_event.just_pressed(KeyCode::KeyW) || key_event.just_pressed(KeyCode::ArrowUp) {
        ui_select_info.menu = (ui_select_info.menu + 2) % 3;
    } else if key_event.just_pressed(KeyCode::KeyS) || key_event.just_pressed(KeyCode::ArrowDown) {
        ui_select_info.menu = (ui_select_info.menu + 1) % 3;
    } else if key_event.just_pressed(KeyCode::Enter) {
        if ui_select_info.menu == 2 {
            next_state.set(GameState::UIMapEditor);
        } else {
            next_state.set(GameState::UIGameEnter);
        }
    }
    for (index, interaction) in query_hover.iter().enumerate() {
        match *interaction {
            Interaction::Pressed => {
                if ui_select_info.menu == 2 {
                    next_state.set(GameState::UIMapEditor);
                } else {
                    next_state.set(GameState::UIGameEnter);
                }
            }
            Interaction::Hovered => {
                ui_select_info.menu = index;
            }
            _ => {}
        }
    }
    for (index, parent) in query_entity.iter().enumerate() {
        if index == ui_select_info.menu {
            commands.set_style(parent.get(), class_node_menu_text_hover);
        } else {
            commands.set_style(parent.get(), class_node_menu_text_default);
        }
    }
    for (index, mut visibility) in query_visible.iter_mut().enumerate() {
        if index == ui_select_info.menu {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
