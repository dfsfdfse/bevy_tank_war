use bevy::prelude::*;

use crate::{
    res::{Clear, GameMapCollection, GAME_ICON_ARROW_LEFT},
    utils::{
        animate::Animator,
        widget::{grid, node_children, node_root, node_text, GridItemInfo},
    },
};

use super::class::editor_class::{
    class_node_icon_text, class_node_left_panel, class_node_map_name_text,
};

pub fn setup_ui_editor(commands: Commands, gm_maps: Res<GameMapCollection>) {
    node_root(class_node_left_panel, commands, Clear, |gc| {
        node_children((), gc, (), |gc| {
            node_children((), gc, (), |gc| {
                node_text(
                    GAME_ICON_ARROW_LEFT,
                    class_node_icon_text,
                    gc,
                    Animator::default(),
                );
                node_text("LEVEL", (), gc, ());
            });
            grid(gm_maps.maps.len(), 1, 30., (), (), gc, (), |gc, r, c| {
                node_text("", class_node_map_name_text, gc, GridItemInfo(r, c));
            });
        });
        node_children((), gc, (), |gc| {
            node_children((), gc, (), |gc| {
                node_text(
                    GAME_ICON_ARROW_LEFT,
                    class_node_icon_text,
                    gc,
                    Animator::default(),
                );
                node_text("BLOCKS", (), gc, ());
            });
            grid(5, 2, 60., (), (), gc, (), |gc, r, c| {});
        });
    });
}

pub fn update_ui_editor() {}
