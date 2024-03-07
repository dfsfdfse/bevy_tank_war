use bevy::prelude::*;

use crate::res::{
    Block, GameDirection, GameSource, Moving, Player, GAME_AREA_BLOCK, GAME_BLOCK_SIZE, GAME_SIZE,
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
    transform.translation.z = if block.block == 4 { 3. } else { 2. };
}

pub fn class_game_update(
    mut transform: Mut<Transform>,
    mut layout: Mut<TextureAtlas>,
    mut tank: Mut<Moving>,
    mut player: Mut<Player>,
) {
    let idx = (layout.index + 1) % 2;
    let player = player.as_mut();
    if let Some(dir) = player.direction_stack.last() {
        tank.direction = *dir;
        match tank.direction {
            GameDirection::Up => {
                layout.index = idx;
                transform.translation.y += tank.speed as f32;
            }
            GameDirection::Down => {
                layout.index = 2 + idx;
                transform.translation.y -= tank.speed as f32;
            }
            GameDirection::Left => {
                layout.index = 4 + idx;
                transform.translation.x -= tank.speed as f32;
            }
            GameDirection::Right => {
                layout.index = 6 + idx;
                transform.translation.x += tank.speed as f32;
            }
        }
        if let Some(last_direction) = player.last_turn_direction {
            if last_direction != *dir {
                player.last_turn_direction = Some(*dir);
                let mod_v = 12.;
                let mod_y = transform.translation.y % mod_v;
                if mod_y > mod_v / 2. {
                    transform.translation.y += mod_v - mod_y;
                } else {
                    transform.translation.y -= mod_y;
                }
                let mod_x = transform.translation.x % mod_v;
                if mod_x > mod_v / 2. {
                    transform.translation.x += mod_v - mod_x;
                } else {
                    transform.translation.x -= mod_x;
                }
            }
        } else {
            player.last_turn_direction = Some(*dir);
        }
    }
}
