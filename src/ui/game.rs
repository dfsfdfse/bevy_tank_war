use bevy::prelude::*;

use crate::{
    res::{Clear, GameMapCollection, UISelectInfo},
    utils::{
        util::is_four,
        widget::{sprite, sprite_root, sprite_sheet},
    },
};

use super::{
    class::game_class::{class_sprite_block, class_sprite_panel, class_sprite_sheet_block},
    widget::wd_sprite_block,
};

pub fn setup_ui_game(
    commands: Commands,
    gm_map: Res<GameMapCollection>,
    ui_map_select: Res<UISelectInfo>,
) {
    sprite_root(class_sprite_panel, commands, Clear, |gc| {
        for block in gm_map.maps[ui_map_select.map_index].to_blocks().iter() {
            if is_four(block.block) {
                sprite_sheet(class_sprite_sheet_block, gc, block.clone());
            } else if [3, 4, 5].contains(&block.block) {
                wd_sprite_block(gc, block);
            } else {
                sprite(class_sprite_block, gc, block.clone());
            }
        }
    });
}
