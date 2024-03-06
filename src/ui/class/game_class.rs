use bevy::prelude::*;

use crate::res::{Block, GameSource, GAME_AREA_BLOCK, GAME_BLOCK_SIZE, GAME_SIZE};

pub fn class_sprite_panel(
    mut sprite: Mut<Sprite>,
    mut image: Mut<Handle<Image>>,
    gm_res: Res<GameSource>,
) {
    *image = gm_res.panel.clone();
    sprite.custom_size = Some(GAME_SIZE);
}

pub fn class_sprite_sheet_block(
    mut image: Mut<Handle<Image>>,
    mut sprite: Mut<Sprite>,
    mut atlas: Mut<TextureAtlas>,
    mut transform: Mut<Transform>,
    block: Mut<Block>,
    gm_res: Res<GameSource>,
) {
    if block.block == 6 {
        atlas.layout = gm_res.layout.clone();
    } else {
        atlas.layout = gm_res.layout_tank.clone();
    }
    let (x, y) = block.to_pos();
    let size = if GAME_AREA_BLOCK.contains(&block.block) {
        1
    } else {
        2
    };
    sprite.custom_size = Some(Vec2::new(
        (GAME_BLOCK_SIZE.1 * size) as f32,
        (GAME_BLOCK_SIZE.0 * size) as f32,
    ));
    *image = gm_res.blocks[block.block - 1].clone();
    transform.translation.x = x;
    transform.translation.y = y;
    transform.translation.z = 2.;
}

pub fn class_sprite_block(
    mut image: Mut<Handle<Image>>,
    mut sprite: Mut<Sprite>,
    mut transform: Mut<Transform>,
    block: Mut<Block>,
    gm_res: Res<GameSource>,
) {
    let size = if GAME_AREA_BLOCK.contains(&block.block) {
        1
    } else {
        2
    };
    sprite.custom_size = Some(Vec2::new(
        (GAME_BLOCK_SIZE.1 * size) as f32,
        (GAME_BLOCK_SIZE.0 * size) as f32,
    ));
    *image = gm_res.blocks[block.block - 1].clone();
    let (x, y) = block.to_pos();
    transform.translation.x = x;
    transform.translation.y = y;
    transform.translation.z = 2.;
}
