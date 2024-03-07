use bevy::prelude::*;

use crate::{
    res::{Clear, GameDirection, GameMapCollection, GameState, Player, UISelectInfo},
    utils::widget::sprite_root,
};

use super::{class::game_class::class_sprite_panel, widget::wd_load_game_map};

pub fn setup_ui_game(
    commands: Commands,
    gm_map: Res<GameMapCollection>,
    ui_map_select: Res<UISelectInfo>,
    gm_state: Res<State<GameState>>,
) {
    sprite_root(class_sprite_panel, commands, Clear, |gc| {
        wd_load_game_map(gc, &gm_map, &ui_map_select, &gm_state);
    });
}

pub fn update_ui_game(
    mut commands: Commands,
    mut query_player: Query<(&mut Player, Entity)>,
    key_event: Res<ButtonInput<KeyCode>>,
) {
    for (mut player, entity) in query_player.iter_mut() {
        if let Some(keys) = player.keys_binding {
            if key_event.just_pressed(keys.up) {
                player.direction_stack.push(GameDirection::Up);
            } else if key_event.just_released(keys.up) {
                player.direction_stack.retain(|&x| x != GameDirection::Up);
            }
            if key_event.just_pressed(keys.down) {
                player.direction_stack.push(GameDirection::Down);
            } else if key_event.just_released(keys.down) {
                player.direction_stack.retain(|&x| x != GameDirection::Down);
            }
            if key_event.just_pressed(keys.left) {
                player.direction_stack.push(GameDirection::Left);
            } else if key_event.just_released(keys.left) {
                player.direction_stack.retain(|&x| x != GameDirection::Left);
            }
            if key_event.just_pressed(keys.right) {
                player.direction_stack.push(GameDirection::Right);
            } else if key_event.just_released(keys.right) {
                player
                    .direction_stack
                    .retain(|&x| x != GameDirection::Right);
            }
        }
    }
}
