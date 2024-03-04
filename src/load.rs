use bevy::prelude::*;
use crate::res::{GameMapCollection, GameState, HandleLoadMap};

pub fn setup_load_game_map_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(HandleLoadMap(asset_server.load("map.ron")));
}

pub fn update_load_to_ui_menu(
    mut commands: Commands,
    map_handle: Res<HandleLoadMap>,
    mut maps: ResMut<Assets<GameMapCollection>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(map) = maps.remove(map_handle.0.id()) {
        commands.insert_resource(map);
        next_state.set(GameState::UIMenu);
    }
}