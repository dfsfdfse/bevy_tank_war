use bevy::prelude::*;

use crate::{
    res::{Clear, GameMapCollection},
    utils::widget::{grid, node_children, node_root, node_text},
};

use super::class::editor_class::class_node_left_panel;

pub fn setup_ui_editor(commands: Commands, gm_maps: Res<GameMapCollection>) {
    node_root(class_node_left_panel, commands, Clear, |gc| {
        node_children((), gc, (), |gc| {
            node_children((), gc, (), |gc| {
                node_text("", (), gc, ());
                node_text("", (), gc, ());
            });
            grid(gm_maps.maps.len(), 1, 30., (), (), gc, (), |gc, r, c| {});
        });
    });
}

pub fn update_ui_editor() {

}
