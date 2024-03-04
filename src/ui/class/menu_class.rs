use bevy::prelude::*;

use crate::{
    res::{
        GameSource, UISelectInfo, GAME_BLOCK_SIZE, GAME_LOGO_SIZE, GAME_MENU_TEXT_SIZE, GAME_SIZE,
    },
    utils::widget::GridItemInfo,
};

pub fn class_node_root(mut style: Mut<Style>) {
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style.width = Val::Percent(100.);
    style.height = Val::Percent(100.);
}

pub fn class_node_game_panel(
    mut style: Mut<Style>,
    mut border_color: Mut<BorderColor>,
    mut background_color: Mut<BackgroundColor>,
) {
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(GAME_SIZE.x);
    style.height = Val::Px(GAME_SIZE.y);
    style.border = UiRect::all(Val::Px(3.));
    border_color.0 = Color::rgb(0.6, 0.6, 0.6);
    background_color.0 = Color::BLACK;
}

pub fn class_node_game_logo_image(
    mut style: Mut<Style>,
    mut image: Mut<UiImage>,
    gm_res: Res<GameSource>,
) {
    style.width = Val::Px(GAME_LOGO_SIZE.0);
    style.height = Val::Px(GAME_LOGO_SIZE.1);
    image.texture = gm_res.logo.clone();
}

pub fn class_node_menu(mut style: Mut<Style>) {
    style.width = Val::Px(400.);
    style.height = Val::Px(360.);
    style.display = Display::Flex;
    style.align_items = AlignItems::Center;
}

pub fn class_node_menu_item(mut style: Mut<Style>, grid_item_info: Mut<GridItemInfo>) {
    match grid_item_info.1 {
        0 => {
            style.width = Val::Px(120.);
            style.justify_content = JustifyContent::End;
            style.padding.right = Val::Px(20.);
        }
        _ => {
            style.justify_content = JustifyContent::Start;
            style.width = Val::Px(250.);
            style.height = Val::Px(40.);
            style.padding = UiRect::all(Val::Px(5.));
            style.padding.left = Val::Px(15.);
            style.padding.top = Val::Px(10.);
        }
    }
}

pub fn class_node_menu_text(mut style: Mut<Style>, mut text: Mut<Text>, gm_res: Res<GameSource>) {
    text.sections[0].style.font = gm_res.font.clone();
    text.sections[0].style.font_size = GAME_MENU_TEXT_SIZE;
    text.sections[0].style.color = Color::WHITE;
    text.justify = JustifyText::Center;
    style.width = Val::Px(200.);
    style.height = Val::Px(30.);
}

pub fn class_node_menu_text_default(mut background_color: Mut<BackgroundColor>) {
    background_color.0 = Color::BLACK;
}

pub fn class_node_menu_text_hover(mut background_color: Mut<BackgroundColor>) {
    background_color.0 = Color::rgba(0.2, 0.2, 0.2, 0.5);
}

pub fn class_node_tank_selector(
    mut style: Mut<Style>,
    mut layout: Mut<TextureAtlas>,
    mut image: Mut<UiImage>,
    mut visibility: Mut<Visibility>,
    gm_res: Res<GameSource>,
    grid_item_info: Mut<GridItemInfo>,
    select_info: Res<UISelectInfo>,
) {
    style.width = Val::Px(GAME_BLOCK_SIZE.0 as f32 * 2.);
    style.height = Val::Px(GAME_BLOCK_SIZE.1 as f32 * 2.);
    layout.layout = gm_res.layout_tank.clone();
    image.texture = gm_res.blocks[6].clone();
    if grid_item_info.0 == select_info.menu {
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}
