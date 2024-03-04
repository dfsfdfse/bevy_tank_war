use bevy::prelude::*;
use bevy_asset_loader::{
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};

use crate::{
    load::{setup_load_game_map_resource, update_load_to_ui_menu},
    res::{Clear, GameMapCollection, GameSettings, GameSource, GameState, UISelectInfo, INITIAL_SETTINGS},
    ui::{
        class::despawn_screen, editor::setup_ui_editor, game::setup_ui_game, menu::{setup_ui_menu, update_ui_menu}
    },
    utils::ron::RonAssetPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(GameSettings::default())
            .insert_resource(UISelectInfo::default())
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: INITIAL_SETTINGS.win_title.into(),
                        resolution: INITIAL_SETTINGS.win_resolution.into(),
                        ..default()
                    }),
                    ..default()
                }),
                RonAssetPlugin::<GameMapCollection>::default(),
            ))
            // 资源加载
            .add_loading_state(
                LoadingState::new(GameState::LoadTexture)
                    .continue_to_state(GameState::LoadResource)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("load.assets.ron")
                    .load_collection::<GameSource>(),
            )
            .add_systems(OnExit(GameState::UIMenu), despawn_screen::<Clear>)
            .add_systems(OnExit(GameState::UIGameEnter), despawn_screen::<Clear>)
            .add_systems(OnExit(GameState::UIMapEditor), despawn_screen::<Clear>)
            .add_systems(Startup, setup_load_game_map_resource)
            .add_systems(
                Update,
                update_load_to_ui_menu.run_if(in_state(GameState::LoadResource)),
            )
            .add_systems(OnEnter(GameState::UIMenu), setup_ui_menu)
            .add_systems(Update, update_ui_menu.run_if(in_state(GameState::UIMenu)))
            .add_systems(OnEnter(GameState::UIGameEnter), setup_ui_game)
            .add_systems(OnEnter(GameState::UIMapEditor), (setup_ui_editor, setup_ui_game));
    }
}