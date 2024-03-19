use std::time::Duration;

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
#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub enum RightPanelButton {
    SaveMap,
    NewMap,
    DeleteMap,
    Back,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone)]
pub struct Moving {
    pub direction: GameDirection,
    pub speed: f32,
    pub run_speed: f32,
}

impl Default for Moving {
    fn default() -> Self {
        Moving {
            direction: GameDirection::Up,
            speed: 0.,
            run_speed: 0.,
        }
    }
}

impl Moving {
    pub fn new(direction: GameDirection, speed: f32) -> Self {
        Moving {
            direction,
            speed,
            run_speed: 0.,
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct Bullet {
    pub index: usize,
    pub level: usize,
    pub tank_pos: (f32, f32),
    pub boom: bool,
}

impl Bullet {
    pub fn new(player: &Player, player_pos: &Transform, player_mov: &Moving) -> Self {
        Bullet {
            index: player.index,
            boom: false,
            level: player.level,
            tank_pos: match player_mov.direction {
                GameDirection::Up => (player_pos.translation.x, player_pos.translation.y + 8.0),
                GameDirection::Down => (player_pos.translation.x, player_pos.translation.y - 8.0),
                GameDirection::Left => (player_pos.translation.x - 8.0, player_pos.translation.y),
                GameDirection::Right => (player_pos.translation.x + 8.0, player_pos.translation.y),
            },
        }
    }
}
#[derive(Clone)]
pub enum EnemyState {
    Search,
    Attack,
    Escape,
}
#[derive(Component, Clone, Default)]
pub struct Enemy {
    pub start_search_pos: Option<(f32, f32)>,
}

#[derive(Component, Clone)]
pub struct Player {
    pub id: i64,
    pub index: usize,
    pub level: usize,
    pub direction_stack: Vec<GameDirection>,
    pub fire: bool,
    pub keys_binding: Option<KeysBinding>,
    pub last_turn_direction: Option<GameDirection>,
    pub bullet: Option<Entity>,
    pub shoot_time: Duration,
}

impl Player {
    pub fn new_player1() -> Self {
        Player {
            id: gen_id(),
            index: 6,
            level: 1,
            fire: false,
            direction_stack: vec![],
            keys_binding: Some(PLAYER1_KEYS),
            last_turn_direction: None,
            bullet: None,
            shoot_time: Duration::from_secs(0),
        }
    }

    pub fn new_player2() -> Self {
        Player {
            id: gen_id(),
            index: 7,
            level: 1,
            fire: false,
            direction_stack: vec![],
            keys_binding: Some(PLAYER2_KEYS),
            last_turn_direction: None,
            bullet: None,
            shoot_time: Duration::from_secs(0),
        }
    }

    pub fn new_enemy(index: usize) -> Self {
        Player {
            id: gen_id(),
            index,
            level: 1,
            fire: false,
            direction_stack: vec![],
            keys_binding: None,
            last_turn_direction: None,
            bullet: None,
            shoot_time: Duration::from_secs(0),
        }
    }

    pub fn is_player1(&self) -> bool {
        self.index == 6
    }

    pub fn is_player2(&self) -> bool {
        self.index == 7
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct Colider {
    pub index: usize,
    pub width: f32,
    pub height: f32,
    pub filter: Vec<usize>,
    pub is_container: bool,
}

impl Colider {
    pub fn new(index: usize, width: f32, height: f32) -> Self {
        Colider {
            index,
            width,
            height,
            filter: Vec::new(),
            is_container: false,
        }
    }

    pub fn is_home(&self) -> bool {
        self.index == 6
    }

    /* pub fn new_bullet() -> Self {
        Colider {
            index: 10,
            width: 12.0,
            height: 12.0,
            filter: vec![5],
            is_container: false,
        }
    } */

    /* pub fn is_bullet(&self) -> bool {
        self.index == 10
    } */

    pub fn add_filter(&mut self, filter: usize) -> &mut Self {
        self.filter.push(filter);
        self
    }

    pub fn container(&mut self) -> &mut Self {
        self.is_container = true;
        self
    }
}
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
#[derive(Clone, PartialEq, Default, Debug, Copy, Eq)]
pub enum BlockOperate {
    Remove,
    Add,
    Change,
    #[default]
    None,
}

#[derive(Component, Clone, Debug, PartialEq, Copy)]
pub struct Block {
    pub row: usize,
    pub col: usize,
    pub block: usize,
    pub operate: BlockOperate,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            row: 99,
            col: 99,
            block: 0,
            operate: BlockOperate::None,
        }
    }
}

impl Block {
    pub fn new(row: usize, col: usize, block: usize) -> Self {
        Block {
            row,
            col,
            block,
            operate: BlockOperate::None,
        }
    }

    pub fn to_pos(&self) -> (f32, f32) {
        if GAME_AREA_BLOCK.contains(&self.block) {
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
    pub current: usize,
}

impl NodeBlock {
    pub fn new(index: usize, type_index: usize) -> Self {
        Self {
            index,
            type_index,
            current: type_index,
        }
    }
}
/* -----------Component--------------- */

/* ---------------Const--------------- */
pub struct InitialSettings {
    ///窗口设置
    pub win_title: &'static str,
    pub win_resolution: (f32, f32),
}

#[derive(Clone, Copy)]
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
pub const GAME_AREA_BLOCK: [usize; 5] = [1, 2, 3, 4, 5];
pub const GAME_AREA_BLOCK_FOUR: [usize; 6] = [6, 7, 8, 9, 10, 11];
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
                } else if [3, 4, 5, 6, 7, 8, 9, 10, 11].contains(block) {
                    blocks.push(Block::new(r, c, *block));
                    stick.push((r, c + 1));
                    stick.push((r + 1, c));
                    stick.push((r + 1, c + 1));
                } else if [1, 2].contains(block) {
                    blocks.push(Block::new(r, c, *block));
                }
            }
        }
        if stick.len() > 0 {
            println!("map data error!");
        }
        blocks
    }

    pub fn init_fixed(&mut self) {
        let mut stick = vec![6, 7, 8, 9, 10, 11];
        for row in self.map.iter_mut() {
            for block in row.iter_mut() {
                if stick.contains(block) {
                    stick.retain(|b| *b != *block);
                }
            }
        }
        println!("{:?}", stick);
        for blk in stick.iter() {
            let mut ir = 0;
            let mut ic = 0;
            loop {
                if GAME_AREA_BLOCK_FOUR.contains(&self.map[ir][ic]) {
                    if ir > 25 {
                        ir = 0;
                        ic += 2;
                    } else {
                        ir += 2;
                    }
                } else {
                    self.map[ir][ic] = *blk;
                    self.map[ir][ic + 1] = *blk;
                    self.map[ir + 1][ic] = *blk;
                    self.map[ir + 1][ic + 1] = *blk;
                    break;
                }
            }
        }
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
#[derive(Resource)]
pub struct UISelectInfo {
    pub menu: usize,                   //菜单index
    pub map_index: usize,              //游戏选择的地图index
    pub map_editor_level_index: usize, //地图编辑器选择的地图index
    pub map_editor_block: usize,       //地图编辑器选择的块index
    pub map_editor_blocks_inner: [[usize; 4]; 2],
    pub map_editor_cursor: (usize, usize), //地图编辑器的光标位置大方块的 (row, col)
    pub show_line: bool,                   //是否显示光标的边框
}

impl Default for UISelectInfo {
    fn default() -> Self {
        Self {
            menu: Default::default(),
            map_index: Default::default(),
            map_editor_level_index: Default::default(),
            map_editor_block: Default::default(),
            map_editor_blocks_inner: [[1; 4], [2; 4]],
            map_editor_cursor: Default::default(),
            show_line: Default::default(),
        }
    }
}
#[derive(Resource, Default)]
pub struct LastSelectInfo {
    pub last_map_editor_block: Option<Entity>,
    pub last_map_editor_small_block: Option<Entity>,
    pub last_spawn_block: Vec<Block>, //地图编辑器点击后上次生成的块 左上位置的块
}

/* -----------Resource--------------- */
