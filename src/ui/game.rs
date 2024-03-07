use bevy::prelude::*;

use crate::{
    res::{Clear, GameMapCollection, GameState, UISelectInfo},
    utils::widget::sprite_root,
};

use super::{class::game_class::class_sprite_panel, widget::wd_load_game_map};

pub fn setup_ui_game(
    commands: Commands,
    gm_map: Res<GameMapCollection>,
    ui_map_select: Res<UISelectInfo>,
    gm_state: Res<State<GameState>>,
) {
    sprite_root(class_sprite_panel, commands, Clear, |gc| {
        wd_load_game_map(gc, &gm_map, &ui_map_select, &gm_state);
    });
}
