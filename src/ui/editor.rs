use bevy::prelude::*;

use crate::{
    res::{Clear, GameMapCollection, NodeBlock, UISelectInfo},
    utils::{
        class::StyleCommand,
        widget::{node_children, node_root, text, GridItemInfo},
    },
};

use super::{
    class::{
        class_node_fill,
        editor_class::{
            class_node_collapse_item_default, class_node_collapse_item_hover,
            class_node_left_panel, class_node_map_name_style, class_node_map_name_text,
        },
    },
    widget::{wd_node_block, wd_setup_collapse_grid},
};

pub fn setup_ui_editor(
    commands: Commands,
    gm_maps: Res<GameMapCollection>,
    select_info: Res<UISelectInfo>,
) {
    node_root(class_node_left_panel, commands, Clear, |gc| {
        wd_setup_collapse_grid("LEVEL", gm_maps.maps.len(), 1, 30., gc, |gc, r, c| {
            node_children(
                (
                    class_node_fill,
                    if select_info.map_editor_level_index == r {
                        class_node_collapse_item_hover
                    } else {
                        class_node_collapse_item_default
                    },
                ),
                gc,
                (Interaction::None, GridItemInfo(r, c)),
                |gc| {
                    node_children(class_node_map_name_style, gc, (), |gc| {
                        text(
                            [gm_maps.maps[r].name.as_str()],
                            class_node_map_name_text,
                            gc,
                            (),
                        );
                    });
                },
            );
        });
        wd_setup_collapse_grid("BLOCK", 5, 2, 75., gc, |gc, r, c| {
            let index = r * 2 + c;
            if index < 9 {
                wd_node_block((Interaction::None, NodeBlock::new(0, index)), gc, r, c, select_info.as_ref());
            }
        });
    });
}

pub fn update_ui_editor(
    mut commands: Commands,
    query_event: Query<(&Interaction, &GridItemInfo), Changed<Interaction>>,
    query_entity: Query<Entity, (With<Interaction>, With<GridItemInfo>)>,
    mut ui_selector: ResMut<UISelectInfo>,
) {
    for (interaction, grid_item) in query_event.iter() {
        if *interaction == Interaction::Pressed {
            ui_selector.map_editor_level_index = grid_item.0;
        }
    }
    for (index, entity) in query_entity.iter().enumerate() {
        if index == ui_selector.map_editor_level_index {
            commands.set_style(entity, class_node_collapse_item_hover);
        } else {
            commands.set_style(entity, class_node_collapse_item_default);
        }
    }
}
