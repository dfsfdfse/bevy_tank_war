use std::{array::from_fn, time::Duration};

use bevy::prelude::*;

use crate::{
    res::{Block, NodeBlock, Relate, UISelectInfo, GAME_ICON_ARROW_LEFT},
    utils::{
        animate::{Animator, LoopStrategy},
        class::StyleCommand,
        widget::{
            atlas_image, grid, image, node_children, sprite, sprite_children, text, GridInfo,
        },
    },
};

use super::class::{
    class_node_fill,
    editor_class::{
        class_node_collapse_background, class_node_collapse_title, class_node_column_align_center,
        class_node_margin_bottom, class_node_padding_left, class_node_text_title,
        class_wd_node_block_container, class_wd_node_block_container_default,
        class_wd_node_block_container_select, class_wd_node_block_contianer_image,
        class_wd_node_block_item, class_wd_node_block_size, class_wd_node_block_size_inner,
    },
    game_class::class_sprite_block,
};

pub fn wd_sprite_block(gc: &mut ChildBuilder, block: &Block, relate: impl Bundle) {
    sprite_children((), gc, relate, |gc| {
        sprite(
            class_sprite_block,
            gc,
            Block::new(block.row, block.col, block.block, (1, 1)),
        );
        sprite(
            class_sprite_block,
            gc,
            Block::new(block.row, block.col + 1, block.block, (1, 1)),
        );
        sprite(
            class_sprite_block,
            gc,
            Block::new(block.row + 1, block.col, block.block, (1, 1)),
        );
        sprite(
            class_sprite_block,
            gc,
            Block::new(block.row + 1, block.col + 1, block.block, (1, 1)),
        );
    });
}
///目前bevy支持的文字组件设置样式确实太垃圾,只能多层嵌套
pub fn wd_setup_collapse_grid(
    title: &str,
    row: usize,
    col: usize,
    item_height: f32,
    gc: &mut ChildBuilder,
    children: impl FnMut(&mut ChildBuilder, usize, usize),
) {
    let relate = Relate::new();
    node_children(
        (class_node_column_align_center, class_node_margin_bottom),
        gc,
        (),
        |gc| {
            node_children(
                (class_node_collapse_title, class_node_collapse_background),
                gc,
                (),
                |gc| {
                    node_children(
                        (
                            class_node_padding_left,
                            class_node_column_align_center,
                            class_node_fill,
                        ),
                        gc,
                        (Interaction::None, relate),
                        |gc| {
                            text(
                                [GAME_ICON_ARROW_LEFT, " ", title],
                                class_node_text_title,
                                gc,
                                (Animator::default(), relate),
                            );
                        },
                    );
                },
            );

            grid(
                row,
                col,
                item_height,
                wd_collapse_container_class,
                (),
                gc,
                (Animator::default(), relate),
                children,
            );
        },
    );
}

fn wd_collapse_container_class(
    mut style: Mut<Style>,
    mut animator: Mut<Animator>,
    grid_info: Res<GridInfo>,
) {
    style.height = Val::Px(0.);
    style.overflow = Overflow::clip();
    style.display = Display::Flex;
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style.flex_wrap = FlexWrap::Wrap;
    let h = grid_info.2 * grid_info.0 as f32;
    animator
        .set_loop_strategy(LoopStrategy::Once)
        .set_pause(true)
        .add_change()
        .set_retain_change(true)
        .set_duration(Duration::from_millis(300))
        .set_style_size(Val::Auto, Val::Px(-h));
    animator
        .add_change()
        .set_retain_change(true)
        .set_duration(Duration::from_millis(300))
        .set_style_size(Val::Auto, Val::Px(h));
}

///根据Relate查询到关联组件,然后根据Interaction的变化来更新动画,实现伸缩动画
pub fn wd_update_collapse_grid(
    mut query_relate: Query<(&Relate, &mut Animator), With<Relate>>,
    query_change: Query<(&Interaction, &Relate), Changed<Interaction>>,
) {
    for (interaction, relate) in query_change.iter() {
        for (r, mut animator) in query_relate.iter_mut() {
            if r == relate && *interaction == Interaction::Pressed {
                let index = animator.get_exec_index();
                if animator.get_pause() {
                    animator.set_pause(false).exec_index((index + 1) % 2);
                }
            }
        }
    }
}

pub fn wd_node_block(
    relate: impl Bundle,
    gc: &mut ChildBuilder,
    row: usize,
    col: usize,
    select_info: &UISelectInfo,
) {
    let index = row * 2 + col;
    if index < 6 {
        node_children(
            (
                class_wd_node_block_container,
                class_wd_node_block_size,
                if select_info.map_editor_block == index {
                    class_wd_node_block_container_select
                } else {
                    class_wd_node_block_container_default
                },
            ),
            gc,
            relate,
            |gc| {
                if [1, 2].contains(&index) {
                    let _a: [Entity; 4] = from_fn(|i| {
                        image(
                            class_wd_node_block_item,
                            gc,
                            (NodeBlock::new(i, index), Interaction::None),
                        )
                    });
                } else {
                    let _a: [Entity; 4] =
                        from_fn(|i| image(class_wd_node_block_item, gc, NodeBlock::new(i, index)));
                }
            },
        );
    } else {
        node_children(
            (
                class_wd_node_block_container,
                class_wd_node_block_size,
                if select_info.map_editor_block == index {
                    class_wd_node_block_container_select
                } else {
                    class_wd_node_block_container_default
                },
            ),
            gc,
            relate,
            |gc| {
                atlas_image(
                    (
                        class_wd_node_block_size_inner,
                        class_wd_node_block_contianer_image,
                    ),
                    gc,
                    NodeBlock::new(0, index),
                );
            },
        );
    }
}

pub fn wd_update_node_block(
    mut commands: Commands,
    query_entity: Query<Entity, (With<Interaction>, With<NodeBlock>, With<BorderColor>)>,
    mut query_change: Query<(&Interaction, &NodeBlock), (Changed<Interaction>, With<BorderColor>)>,
    mut ui_select_info: ResMut<UISelectInfo>,
) {
    for (interaction, node_block) in query_change.iter_mut() {
        if *interaction == Interaction::Pressed {
            ui_select_info.map_editor_block = node_block.type_index;
        }
    }
    for (index, entity) in query_entity.iter().enumerate() {
        if index == ui_select_info.map_editor_block {
            commands.set_style(entity, class_wd_node_block_container_select);
        } else {
            commands.set_style(entity, class_wd_node_block_container_default);
        }
    }
}
