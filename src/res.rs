use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use serde::{Deserialize, Serialize};

use crate::plugins::gen_id;

///全局游戏状态
#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    LoadTexture,
    LoadResource,
    UIMenu,
    UIGameEnter,
    Gaming,
    UIMapEditor,
}
/* -----------Component--------------- */
///清除实体的组件
#[derive(Component)]
pub struct Clear;
///关联查询的组件
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Relate(pub i64);

impl Relate {
    pub fn new() -> Self {
        Relate(gen_id())
    }
}

#[derive(Component, Clone, Debug)]
pub struct Block {
    pub row: usize,
    pub col: usize,
    pub block: usize,
    pub size: (usize, usize),
}

impl Block {
    pub fn new(row: usize, col: usize, block: usize, size: (usize, usize)) -> Self {
        Block {
            row,
            col,
            block,
            size,
        }
    }

    pub fn to_pos(&self) -> (f32, f32) {
        if self.size == (1, 1) {
            (
                (self.col as f32 - 12.5) * GAME_BLOCK_SIZE.1 as f32,
                (12.5 - self.row as f32) * GAME_BLOCK_SIZE.0 as f32,
            )
        } else {
            (
                (self.col as f32 - 12.) * GAME_BLOCK_SIZE.1 as f32,
                (12. - self.row as f32) * GAME_BLOCK_SIZE.0 as f32,
            )
        }
    }
}

#[derive(Component, Clone)]
pub struct NodeBlock {
    pub index: usize,
    pub type_index: usize,
}

impl NodeBlock {
    pub fn new(index: usize, type_index: usize) -> Self {
        Self { index, type_index }
    }
}
/* -----------Component--------------- */

/* ---------------Const--------------- */
pub struct InitialSettings {
    ///窗口设置
    pub win_title: &'static str,
    pub win_resolution: (f32, f32),
}

pub struct KeysBinding {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode,
}

pub const INITIAL_SETTINGS: InitialSettings = InitialSettings {
    win_resolution: (1080., 720.),
    win_title: "Tank War",
};
///默认玩家1的按键设置
pub const PLAYER1_KEYS: KeysBinding = KeysBinding {
    up: KeyCode::KeyW,
    down: KeyCode::KeyS,
    left: KeyCode::KeyA,
    right: KeyCode::KeyD,
    fire: KeyCode::KeyJ,
};
///默认玩家2的按键设置
pub const PLAYER2_KEYS: KeysBinding = KeysBinding {
    up: KeyCode::ArrowUp,
    down: KeyCode::ArrowDown,
    left: KeyCode::ArrowLeft,
    right: KeyCode::ArrowRight,
    fire: KeyCode::NumpadEnter,
};

pub const MENU_LIST: [&str; 3] = ["1 player", "2 players", "editor"];
pub const GAME_SIZE: Vec2 = Vec2::new(630., 630.);
pub const GAME_BLOCK_SIZE: (usize, usize) = (24, 24);
pub const GAME_LOGO_SIZE: (f32, f32) = (450., 120.);
pub const GAME_MENU_TEXT_SIZE: f32 = 22.0;
pub const GAME_ICON_ARROW_LEFT: &'static str = "\u{e7f9}";
pub const GAME_ICON_ARROW_DOWN: &'static str = "\u{e873}";
/* ---------------Const--------------- */

/* -----------Resource--------------- */
///全局游戏设置信息
#[derive(Resource)]
pub struct GameSettings {
    pub keys_binding: (KeysBinding, KeysBinding),
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            keys_binding: (PLAYER1_KEYS, PLAYER2_KEYS),
        }
    }
}
///游戏所需的资源
#[derive(AssetCollection, Resource)]
pub struct GameSource {
    #[asset(key = "layout")]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(key = "layout_tank")]
    pub layout_tank: Handle<TextureAtlasLayout>,
    #[asset(key = "logo")]
    pub logo: Handle<Image>,
    #[asset(key = "panel")]
    pub panel: Handle<Image>,
    #[asset(key = "blocks", collection(typed))]
    pub blocks: Vec<Handle<Image>>,
    #[asset(key = "bullets", collection(typed))]
    pub bullets: Vec<Handle<Image>>,
    #[asset(key = "font")]
    pub font: Handle<Font>,
    #[asset(key = "font_mono")]
    pub font_mono: Handle<Font>,
    #[asset(key = "font_icon")]
    pub font_icon: Handle<Font>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct GameMap {
    pub map: Vec<Vec<usize>>,
    pub name: String,
}

impl GameMap {
    pub fn to_blocks(&self) -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];
        let mut stick = vec![];
        for (r, row) in self.map.iter().enumerate() {
            for (c, block) in row.iter().enumerate() {
                if stick.contains(&(r, c)) {
                    stick.retain(|(sy, sx)| !(*sy == r && *sx == c));
                } else if [3, 4, 5, 6, 7].contains(block) {
                    blocks.push(Block::new(r, c, *block, (2, 2)));
                    stick.push((r, c + 1));
                    stick.push((r + 1, c));
                    stick.push((r + 1, c + 1));
                } else if [1, 2].contains(block) {
                    blocks.push(Block::new(r, c, *block, (1, 1)));
                }
            }
        }
        if stick.len() > 0 {
            println!("map data error!");
        }
        blocks
    }
}

#[derive(Asset, TypePath, Resource, Default, Deserialize, Serialize, Clone)]
pub struct GameMapCollection {
    pub maps: Vec<GameMap>,
}
///加载地图资源，加载完成后就没有了
#[derive(Resource)]
pub struct HandleLoadMap(pub Handle<GameMapCollection>);
///界面中选择的信息
#[derive(Resource, Default)]
pub struct UISelectInfo {
    pub menu: usize,//菜单index
    pub map_index: usize,//游戏选择的地图index
    pub map_editor_level_index: usize, //地图编辑器选择的地图index
    pub map_editor_block: usize,//地图编辑器选择的块index
    pub map_editor_cursor: (usize, usize),//地图编辑器的光标位置大方块的 (row, col)
    pub show_line: bool,//是否显示光标的边框
}

/* -----------Resource--------------- */
