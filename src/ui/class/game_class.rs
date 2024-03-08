use bevy::prelude::*;

use crate::res::{
    Block, Bullet, GameDirection, GameSource, Moving, GAME_AREA_BLOCK, GAME_BLOCK_SIZE, GAME_SIZE,
};

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
    transform.translation.z = 3.;
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
    transform.translation.z = if block.block == 4 { 4. } else { 2. };
}

pub fn class_sprite_bullet(
    mut image: Mut<Handle<Image>>,
    mut transform: Mut<Transform>,
    direction: Mut<Moving>,
    bullet: Mut<Bullet>,
    gm_res: Res<GameSource>,
) {
    match direction.direction {
        GameDirection::Up => {
            transform.translation = Vec3::new(bullet.tank_pos.0, bullet.tank_pos.1 + 24., 3.);
            *image = gm_res.bullets[0].clone();
        }
        GameDirection::Down => {
            transform.translation = Vec3::new(bullet.tank_pos.0, bullet.tank_pos.1 - 24., 3.);
            *image = gm_res.bullets[1].clone();
        }
        GameDirection::Left => {
            transform.translation = Vec3::new(bullet.tank_pos.0 - 24., bullet.tank_pos.1, 3.);
            *image = gm_res.bullets[2].clone();
        }
        GameDirection::Right => {
            transform.translation = Vec3::new(bullet.tank_pos.0 + 24., bullet.tank_pos.1, 3.);
            *image = gm_res.bullets[3].clone();
        }
    }
}
