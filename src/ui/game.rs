use bevy::prelude::*;

use crate::{
    res::{
        Clear, Colider, GameDirection, GameMapCollection, GameState, Moving, Player, UISelectInfo,
    },
    utils::{class::StyleCommand, widget::sprite_root},
};

use super::{
    class::game_class::{class_game_update, class_sprite_panel},
    widget::wd_load_game_map,
};

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
            commands.set_style(entity, class_game_update);
        }
    }
}

pub fn update_check_collision(
    mut query_movable: Query<(&mut Transform, &Colider, &Moving), With<Moving>>,
    query_colider: Query<(&Transform, &Colider), Without<Moving>>,
) {
    for (mut transform, rect, mov) in query_movable.iter_mut() {
        for (st_transform, collider) in query_colider.iter() {
            if collider.is_container {
                match mov.direction {
                    GameDirection::Up => {
                        if transform.translation.y + rect.height / 2.0 > collider.height / 2.0 {
                            transform.translation.y = collider.height / 2.0 - rect.height / 2.0;
                        }
                    }
                    GameDirection::Down => {
                        if transform.translation.y - rect.height / 2.0 < -collider.height / 2.0 {
                            transform.translation.y = -collider.height / 2.0 + rect.height / 2.0;
                        }
                    }
                    GameDirection::Left => {
                        if transform.translation.x - rect.width / 2.0 < -collider.width / 2.0 {
                            transform.translation.x = -collider.width / 2.0 + rect.width / 2.0;
                        }
                    }
                    GameDirection::Right => {
                        if transform.translation.x + rect.width / 2.0 > collider.width / 2.0 {
                            transform.translation.x = collider.width / 2.0 - rect.width / 2.0;
                        }
                    }
                }
            } else {
                if transform.translation.y + rect.height / 2.0
                > st_transform.translation.y - collider.height / 2.0
                && transform.translation.y - rect.height / 2.0
                    < st_transform.translation.y + collider.height / 2.0
                && transform.translation.x + rect.width / 2.0
                    > st_transform.translation.x - collider.width / 2.0
                && transform.translation.x - rect.width / 2.0
                    < st_transform.translation.x + collider.width / 2.0
            {
                match mov.direction {
                    GameDirection::Up => {
                        transform.translation.y =
                            st_transform.translation.y - collider.height / 2.0 - rect.height / 2.0;
                    }
                    GameDirection::Down => {
                        transform.translation.y =
                            st_transform.translation.y + collider.height / 2.0 + rect.height / 2.0;
                    }
                    GameDirection::Left => {
                        transform.translation.x =
                            st_transform.translation.x + collider.width / 2.0 + rect.width / 2.0;
                    }
                    GameDirection::Right => {
                        transform.translation.x =
                            st_transform.translation.x - collider.width / 2.0 - rect.width / 2.0;
                    }
                }
            }
            }
            
        }
    }
}
