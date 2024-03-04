use std::time::Duration;
use bevy::prelude::*;
use crate::{
    res::{GameMapCollection, GameSource},
    utils::{animate::{Animator, LoopStrategy}, widget::GridItemInfo},
};

pub fn class_node_left_panel(mut style: Mut<Style>, mut background_color: Mut<BackgroundColor>) {
    style.width = Val::Px(150.);
    style.height = Val::Percent(100.);
    style.display = Display::Flex;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    background_color.0 = Color::rgb_u8(30, 34, 41);
}

pub fn class_node_icon_text(
    mut text: Mut<Text>,
    gm_res: Res<GameSource>,
    mut animator: Mut<Animator>,
) {
    text.sections[0].style.font = gm_res.font_icon.clone();
    text.sections[0].style.font_size = 12.0;
    text.sections[0].style.color = Color::WHITE;
    animator
        .set_loop_strategy(LoopStrategy::Once)
        .set_pause(true)
        .add_change()
        .set_retain_change(true)
        .set_duration(Duration::from_millis(300))
        .set_text_value("\u{e7f9}".to_string())
        .set_text_color(Color::rgba(0., 0., 0., -0.5));
    animator
        .add_change()
        .set_retain_change(true)
        .set_duration(Duration::from_millis(300))
        .set_text_value("\u{e873}".to_string())
        .set_text_color(Color::rgba(0., 0., 0., 0.5));
}

pub fn class_node_map_name_text(
    mut text: Mut<Text>,
    gm_res: Res<GameSource>,
    gm_maps: Res<GameMapCollection>,
    grid_item_info: Mut<GridItemInfo>
) {
    text.sections[0].style.font = gm_res.font_mono.clone();
    text.sections[0].style.font_size = 20.0;
    text.sections[0].style.color = Color::WHITE;
    text.sections[0].value = gm_maps.maps[grid_item_info.0].name.clone();
}