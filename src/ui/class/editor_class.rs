use bevy::prelude::*;

pub fn class_node_left_panel(mut style: Mut<Style>, mut background_color: Mut<BackgroundColor>){
    style.width = Val::Px(150.);
    style.height = Val::Percent(100.);
    style.display = Display::Flex;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    background_color.0 = Color::rgb_u8(30, 34, 41);
}