use bevy::prelude::*;

use crate::{
    res::Block,
    utils::widget::{sprite, sprite_children},
};

use super::class::game_class::class_sprite_block;

pub fn wd_block(gc: &mut ChildBuilder, block: &Block, relate: impl Bundle) {
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
