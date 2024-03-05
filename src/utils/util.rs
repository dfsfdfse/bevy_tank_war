use bevy::prelude::*;

use crate::res::INITIAL_SETTINGS;
///将鼠标在屏幕上的坐标转换为世界坐标
pub fn vec2_to_transform_pos(pos: Vec2) -> (f32, f32) {
    (
        pos.x - INITIAL_SETTINGS.win_resolution.0 / 2.,
        INITIAL_SETTINGS.win_resolution.1 / 2. - pos.y,
    )
}
