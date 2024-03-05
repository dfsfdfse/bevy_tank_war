use crate::{
    res::{GameSource, NodeBlock},
    utils::animate::{Animator, LoopStrategy},
};
use bevy::prelude::*;
use std::time::Duration;

pub fn class_node_left_panel(mut style: Mut<Style>, mut background_color: Mut<BackgroundColor>) {
    style.width = Val::Px(150.);
    style.height = Val::Percent(100.);
    style.display = Display::Flex;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    background_color.0 = Color::rgb_u8(30, 34, 41);
}

pub fn class_node_column_align_center(mut style: Mut<Style>) {
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
}

pub fn class_node_collapse_title(mut style: Mut<Style>) {
    style.width = Val::Px(150.);
    style.height = Val::Px(30.);
}

pub fn class_node_margin_bottom(mut style: Mut<Style>) {
    style.margin.bottom = Val::Px(3.);
}

pub fn class_node_padding_left(mut style: Mut<Style>) {
    style.padding.left = Val::Px(6.);
}

pub fn class_node_collapse_background(mut background_color: Mut<BackgroundColor>) {
    background_color.0 = Color::rgb_u8(120, 120, 120);
}

pub fn class_node_collapse_item_hover(mut background_color: Mut<BackgroundColor>) {
    background_color.0 = Color::RED;
}

pub fn class_node_collapse_item_default(mut background_color: Mut<BackgroundColor>) {
    background_color.0 = Color::rgb_u8(204, 204, 204);
}

pub fn class_node_text_title(
    mut style: Mut<Style>,
    mut text: Mut<Text>,
    gm_res: Res<GameSource>,
    mut animator: Mut<Animator>,
) {
    style.display = Display::Flex;
    style.align_items = AlignItems::Center;
    text.justify = JustifyText::Center;
    text.sections[0].style.font = gm_res.font_icon.clone();
    text.sections[2].style.font = gm_res.font_mono.clone();
    text.sections[2].style.font_size = 20.0;
    animator
        .set_loop_strategy(LoopStrategy::Once)
        .set_pause(true)
        .add_change()
        .set_retain_change(true)
        .set_duration(Duration::from_millis(300))
        .add_text()
        .set_text(String::from("\u{e7f9}"))
        .set_color(Color::rgba(0., 0., 0., -0.5));
    animator
        .add_change()
        .set_retain_change(true)
        .set_duration(Duration::from_millis(300))
        .add_text()
        .set_text(String::from("\u{e873}"))
        .set_color(Color::rgba(0., 0., 0., 0.5));
}

pub fn class_node_map_name_style(mut style: Mut<Style>) {
    style.padding.left = Val::Px(22.);
    style.width = Val::Px(150.);
    style.height = Val::Px(30.);
    style.display = Display::Flex;
    style.align_items = AlignItems::Center;
}

pub fn class_node_map_name_text(
    mut style: Mut<Style>,
    mut text: Mut<Text>,
    gm_res: Res<GameSource>,
) {
    style.width = Val::Px(150.);
    style.height = Val::Px(30.);
    text.sections[0].style.font = gm_res.font_mono.clone();
    text.sections[0].style.font_size = 22.0;
    text.sections[0].style.color = Color::BLACK;
    text.justify = JustifyText::Left;
}

pub fn class_wd_node_block_size(mut style: Mut<Style>) {
    style.width = Val::Px(52.);
    style.height = Val::Px(52.);
}

pub fn class_wd_node_block_size_inner(mut style: Mut<Style>) {
    style.width = Val::Px(48.);
    style.height = Val::Px(48.);
}

pub fn class_wd_node_block_container(mut style: Mut<Style>) {
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.flex_wrap = FlexWrap::Wrap;
    style.border = UiRect::all(Val::Px(2.));
}

pub fn class_wd_node_block_container_select(mut border_color: Mut<BorderColor>) {
    border_color.0 = Color::RED;
}

pub fn class_wd_node_block_container_default(mut border_color: Mut<BorderColor>) {
    border_color.0 = Color::NONE;
}

pub fn class_wd_node_block_contianer_image(
    mut image: Mut<UiImage>,
    mut layout: Mut<TextureAtlas>,
    gm_res: Res<GameSource>,
    node_block: Mut<NodeBlock>,
) {
    let idx = node_block.type_index - 1;
    if idx == 5 {
        layout.layout = gm_res.layout.clone();
    } else {
        layout.layout = gm_res.layout_tank.clone();
    }
    image.texture = gm_res.blocks[idx].clone();
}

pub fn class_wd_node_block_item(
    mut image: Mut<UiImage>,
    mut style: Mut<Style>,
    mut background_color: Mut<BackgroundColor>,
    node_block: Mut<NodeBlock>,
    gm_res: Res<GameSource>,
) {
    style.width = Val::Px(24.);
    style.height = Val::Px(24.);
    if node_block.type_index == 0 {
        background_color.0 = Color::BLACK;
    } else {
        image.texture = gm_res.blocks[node_block.type_index - 1].clone();
    }
}
