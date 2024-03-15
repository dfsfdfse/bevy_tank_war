use bevy::prelude::*;

use crate::{
    res::{
        Block, BlockOperate, Clear, GameMapCollection, GameState, LastSelectInfo, NodeBlock,
        RightPanelButton, UISelectInfo, GAME_AREA_BLOCK_FOUR,
    },
    utils::{
        class::StyleCommand,
        util::{is_four, save_map, vec2_to_transform_pos},
        widget::{button_children, node_children, node_root, sprite, text, GridItemInfo},
    },
};

use super::{
    class::{
        class_node_fill,
        editor_class::{
            class_node_collapse_item_default, class_node_collapse_item_hover,
            class_node_left_panel, class_node_map_name_style, class_node_map_name_text,
            class_node_menu_btn, class_node_menu_btn_text, class_node_right_panel,
        },
        game_class::{class_sprite_block, class_sprite_sheet_block},
    },
    widget::{wd_load_game_map, wd_node_block, wd_setup_collapse_grid},
};

pub fn setup_ui_editor(
    mut commands: Commands,
    gm_maps: Res<GameMapCollection>,
    mut select_info: ResMut<UISelectInfo>,
    mut last_select_info: ResMut<LastSelectInfo>,
) {
    node_root(class_node_left_panel, commands.reborrow(), Clear, |gc| {
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
        wd_setup_collapse_grid("BLOCK", 6, 2, 75., gc, |gc, r, c| {
            let index = r * 2 + c;
            if index < 12 {
                //解决初始化选择时存储
                if index == select_info.map_editor_block {
                    let id = wd_node_block(
                        (Interaction::None, NodeBlock::new(0, index)),
                        gc,
                        r,
                        c,
                        select_info.as_mut(),
                    );
                    last_select_info.last_map_editor_block = Some(id);
                } else {
                    wd_node_block(
                        (Interaction::None, NodeBlock::new(0, index)),
                        gc,
                        r,
                        c,
                        select_info.as_ref(),
                    );
                }
            }
        });
    });
    node_root(class_node_right_panel, commands.reborrow(), Clear, |gc| {
        button_children(class_node_menu_btn, gc, RightPanelButton::NewMap, |gc| {
            text(
                ["new map"],
                class_node_menu_btn_text,
                gc,
                (),
            );
        });
        button_children(class_node_menu_btn, gc, RightPanelButton::DeleteMap, |gc| {
            text(
                ["delete map"],
                class_node_menu_btn_text,
                gc,
                (),
            );
        });
        button_children(class_node_menu_btn, gc, RightPanelButton::SaveMap, |gc| {
            text(
                ["save map"],
                class_node_menu_btn_text,
                gc,
                (),
            );
        });
        button_children(class_node_menu_btn, gc, RightPanelButton::Back, |gc| {
            text(
                ["back menu"],
                class_node_menu_btn_text,
                gc,
                (),
            );
        });
    });
}

pub fn update_ui_editor(
    mut commands: Commands,
    query_event: Query<(&Interaction, &GridItemInfo), Changed<Interaction>>,
    query_entity: Query<Entity, (With<Interaction>, With<GridItemInfo>)>,
    gm_map: Res<GameMapCollection>,
    mut ui_selector: ResMut<UISelectInfo>,
    right_panel_button: Query<(&Interaction, &RightPanelButton), Changed<Interaction>>,
    gm_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    gm_panel_entity: Query<Entity, (With<Sprite>, With<Clear>)>,
) {
    let gm_panel = gm_panel_entity.single();
    for (interaction, grid_item) in query_event.iter() {
        if *interaction == Interaction::Pressed && ui_selector.map_editor_level_index != grid_item.0
        {
            println!("选择地图:{}", grid_item.0);
            ui_selector.map_editor_level_index = grid_item.0;
            commands
                .entity(gm_panel)
                .despawn_descendants()
                .with_children(|gc| {
                    wd_load_game_map(gc, gm_map.as_ref(), &ui_selector, &gm_state);
                });
        }
    }
    //优化：存储上次的选择的实体进行更新
    for (index, entity) in query_entity.iter().enumerate() {
        if index == ui_selector.map_editor_level_index {
            commands.set_style(entity, class_node_collapse_item_hover);
        } else {
            commands.set_style(entity, class_node_collapse_item_default);
        }
    }

    for (interaction, button) in right_panel_button.iter() {
        if *interaction == Interaction::Pressed {
            match *button {
                RightPanelButton::NewMap => {
                    println!("新建地图");
                }
                RightPanelButton::SaveMap => {
                    save_map(&gm_map);
                    println!("保存地图");
                }
                RightPanelButton::Back => {
                    next_state.set(GameState::UIMenu);
                    println!("返回菜单");
                }
                _ => {}
            }
        }
    }
}

pub fn update_ui_editor_brush(
    mut commands: Commands,
    mouse_event: Res<ButtonInput<MouseButton>>,
    mut ui_selector: ResMut<UISelectInfo>,
    mut move_event: EventReader<CursorMoved>,
    mut gizmos: Gizmos,
    mut query_block: Query<(&mut Block, Entity)>,
    mut gm_maps: ResMut<GameMapCollection>,
    gm_panel_entity: Query<Entity, (With<Sprite>, With<Clear>)>,
) {
    let panel_entity = gm_panel_entity.single();
    for evt in move_event.read() {
        let transform_pos = vec2_to_transform_pos(evt.position);
        if transform_pos.0 > -312.
            && transform_pos.1 > -312.
            && transform_pos.1 < 312.
            && transform_pos.0 < 312.
        {
            ui_selector.map_editor_cursor = (
                ((312. - transform_pos.1) / 48.) as usize,
                ((transform_pos.0 + 312.) / 48.) as usize,
            );
            ui_selector.show_line = true;
        } else {
            ui_selector.show_line = false;
        }
    }
    if ui_selector.show_line {
        let (x_start, x_end, y_start, y_end, start_edge, end_edge) = (
            ui_selector.map_editor_cursor.1 as f32 * 48. - 312.,
            ui_selector.map_editor_cursor.1 as f32 * 48. - 264.,
            -(ui_selector.map_editor_cursor.0 as f32 * 48.) + 312.,
            -(ui_selector.map_editor_cursor.0 as f32 * 48.) + 264.,
            -312.,
            312.,
        );
        gizmos.line_2d(
            Vec2::new(x_start, start_edge),
            Vec2::new(x_start, end_edge),
            Color::RED,
        );
        gizmos.line_2d(
            Vec2::new(x_end, start_edge),
            Vec2::new(x_end, end_edge),
            Color::RED,
        );
        gizmos.line_2d(
            Vec2::new(start_edge, y_start),
            Vec2::new(end_edge, y_start),
            Color::RED,
        );
        gizmos.line_2d(
            Vec2::new(start_edge, y_end),
            Vec2::new(end_edge, y_end),
            Color::RED,
        );
        if mouse_event.pressed(MouseButton::Left) {
            let mut operator = vec![];
            if GAME_AREA_BLOCK_FOUR.contains(&ui_selector.map_editor_block) {
                let row = ui_selector.map_editor_cursor.0 * 2;
                let col = ui_selector.map_editor_cursor.1 * 2;
                let editor_block = gm_maps.maps[ui_selector.map_editor_level_index].map[row][col];
                if is_four(editor_block) {
                    return;
                }
                for i in 0..4 {
                    let r = row + (i / 2) as usize;
                    let c = col + (i % 2) as usize;
                    let gm_block = gm_maps.maps[ui_selector.map_editor_level_index].map[r][c];
                    if gm_block != 0 {
                        let mut block = Block::new(r, c, 0);
                        block.operate = BlockOperate::Remove;
                        operator.push(block);
                    }
                }
                //移动
                let mut block = Block::new(row, col, ui_selector.map_editor_block);
                block.operate = BlockOperate::Change;
                operator.push(block);
            } else {
                if GAME_AREA_BLOCK_FOUR.contains(
                    &gm_maps.maps[ui_selector.map_editor_level_index].map
                        [ui_selector.map_editor_cursor.0 * 2][ui_selector.map_editor_cursor.1 * 2],
                ) {
                    return;
                }
                let select_block = if [1, 2].contains(&ui_selector.map_editor_block) {
                    ui_selector.map_editor_blocks_inner[ui_selector.map_editor_block - 1]
                } else {
                    [ui_selector.map_editor_block; 4]
                };
                for (i, blk) in select_block.iter().enumerate() {
                    let r = ui_selector.map_editor_cursor.0 * 2 + (i / 2) as usize;
                    let c = ui_selector.map_editor_cursor.1 * 2 + (i % 2) as usize;
                    let gm_block = gm_maps.maps[ui_selector.map_editor_level_index].map[r][c];
                    if *blk == gm_block {
                        continue;
                    }
                    let mut block = Block::new(r, c, *blk);
                    if *blk == 0 {
                        //删除
                        block.operate = BlockOperate::Remove;
                    } else if gm_block == 0 {
                        //添加
                        block.operate = BlockOperate::Add;
                    } else {
                        //替换
                        block.operate = BlockOperate::Change;
                    }
                    operator.push(block);
                }
            }
            //添加
            for block in operator.iter() {
                if block.operate == BlockOperate::Add {
                    gm_maps.maps[ui_selector.map_editor_level_index].map[block.row][block.col] =
                        block.block;
                    commands.entity(panel_entity).with_children(|gc| {
                        sprite(class_sprite_block, gc, block.clone());
                    });
                }
            }
            for (mut block, entity) in query_block.iter_mut() {
                //todo 优化跳过
                for b in operator.iter_mut() {
                    if b.operate == BlockOperate::Remove && block.row == b.row && block.col == b.col
                    {
                        println!("删除");
                        block.block = b.block;
                        gm_maps.maps[ui_selector.map_editor_level_index].map[block.row]
                            [block.col] = block.block;
                        commands.entity(entity).despawn_recursive();
                        break;
                    }
                    if BlockOperate::Change == b.operate {
                        if block.row == b.row && block.col == b.col {
                            println!("替换");
                            block.block = b.block;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row]
                                [block.col] = b.block;
                            commands.set_style(entity, class_sprite_block);
                            break;
                        }
                        if is_four(b.block) && block.block == b.block {
                            //移动
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row]
                                [block.col] = 0;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row]
                                [block.col + 1] = 0;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row + 1]
                                [block.col] = 0;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row + 1]
                                [block.col + 1] = 0;
                            block.row = b.row;
                            block.col = b.col;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row]
                                [block.col] = block.block;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row]
                                [block.col + 1] = block.block;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row + 1]
                                [block.col] = block.block;
                            gm_maps.maps[ui_selector.map_editor_level_index].map[block.row + 1]
                                [block.col + 1] = block.block;
                            commands.set_style(entity, class_sprite_sheet_block);
                            break;
                        }
                    }
                }
            }
        }
    }
}
