use bevy::prelude::*;
use bevy_asset_loader::{
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};
use idgenerator::{IdGeneratorOptions, IdInstance};

use crate::{
    load::{setup_load_game_map_resource, update_load_to_ui_menu},
    res::{
        Clear, GameMapCollection, GameSettings, GameSource, GameState, LastSelectInfo, UISelectInfo, INITIAL_SETTINGS
    },
    ui::{
        class::despawn_screen,
        editor::{setup_ui_editor, update_ui_editor, update_ui_editor_brush},
        game::setup_ui_game,
        menu::{setup_ui_menu, update_ui_menu},
        widget::{wd_update_collapse_grid, wd_update_node_block},
    },
    utils::{animate::AnimatorPlugin, ron::RonAssetPlugin},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let opt = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
        let _ = IdInstance::set_options(opt);
        app.init_state::<GameState>()
            .insert_resource(GameSettings::default())
            .insert_resource(UISelectInfo::default())
            .insert_resource(LastSelectInfo::default())
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
                AnimatorPlugin,
            ))
            // 资源加载
            .add_loading_state(
                LoadingState::new(GameState::LoadTexture)
                    .continue_to_state(GameState::LoadResource)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("load.assets.ron")
                    .load_collection::<GameSource>(),
            )
            .add_systems(Startup, setup_camera)
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
            .add_systems(
                OnEnter(GameState::UIMapEditor),
                (setup_ui_editor, setup_ui_game),
            )
            .add_systems(
                Update,
                (
                    update_ui_editor,
                    wd_update_collapse_grid,
                    wd_update_node_block,
                    update_ui_editor_brush,
                )
                    .run_if(in_state(GameState::UIMapEditor)),
            );
    }
}

pub fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}
///生成id用于关联查询
pub fn gen_id() -> i64 {
    IdInstance::next_id()
}
