use bevy::prelude::*;

use crate::res::{GAME_AREA_BLOCK, GAME_AREA_BLOCK_FOUR, INITIAL_SETTINGS};
///将鼠标在屏幕上的坐标转换为世界坐标
pub fn vec2_to_transform_pos(pos: Vec2) -> (f32, f32) {
    (
        pos.x - INITIAL_SETTINGS.win_resolution.0 / 2.,
        INITIAL_SETTINGS.win_resolution.1 / 2. - pos.y,
    )
}

pub fn is_four(type_index: usize) -> bool {
    GAME_AREA_BLOCK_FOUR.contains(&type_index)
}

pub fn is_four_or_zero(type_index: usize) -> bool {
    GAME_AREA_BLOCK_FOUR.contains(&type_index) || type_index == 0
}

pub fn is_small(type_index: usize) -> bool {
    GAME_AREA_BLOCK.contains(&type_index)
}

pub fn is_same_size_block(a: usize, b: usize) -> bool {
    (is_four(a) && is_four(b)) || (is_small(a) && is_small(b))
}
