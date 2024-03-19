use std::collections::{BinaryHeap, HashMap};

use bevy::prelude::*;
use serde_ron::to_string;

use crate::res::{
    GameDirection, GameMapCollection, GAME_AREA_BLOCK, GAME_AREA_BLOCK_FOUR, INITIAL_SETTINGS,
};
///将鼠标在屏幕上的坐标转换为世界坐标
pub fn vec2_to_transform_pos(pos: Vec2) -> (f32, f32) {
    (
        pos.x - INITIAL_SETTINGS.win_resolution.0 / 2.,
        INITIAL_SETTINGS.win_resolution.1 / 2. - pos.y,
    )
}

pub fn is_four(type_index: usize) -> bool {
    GAME_AREA_BLOCK_FOUR.contains(&type_index)
}

pub fn is_four_or_zero(type_index: usize) -> bool {
    GAME_AREA_BLOCK_FOUR.contains(&type_index) || type_index == 0
}

pub fn is_small(type_index: usize) -> bool {
    GAME_AREA_BLOCK.contains(&type_index)
}

pub fn is_same_size_block(a: usize, b: usize) -> bool {
    (is_four(a) && is_four(b)) || (is_small(a) && is_small(b))
}

pub fn save_map(map: &GameMapCollection) {
    let map_str = to_string(map).unwrap();
    std::fs::write("assets/map.ron", map_str).unwrap();
}

pub fn transform_to_pos(transform: &Transform) -> (usize, usize) {
    (
        ((300. - transform.translation.y) / 24.) as usize,
        ((300. + transform.translation.x) / 24.) as usize,
    )
}

pub fn path_to_move_direction(path: Vec<(usize, usize)>) -> Option<GameDirection> {
    if path.len() >= 2 {
        return match (
            path[0].0 as isize - path[1].0 as isize,
            path[0].1 as isize - path[1].1 as isize,
        ) {
            (0, 1) => Some(GameDirection::Left),
            (0, -1) => Some(GameDirection::Right),
            (1, 0) => Some(GameDirection::Up),
            (-1, 0) => Some(GameDirection::Down),
            _ => None,
        };
    }
    None
}

/* ---------------------- */

fn can_pass(blk_type: usize) -> bool {
    [0, 3, 4, 6, 9, 10, 11].contains(&blk_type)
}

//曼哈顿距离
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

fn get_neighbors(
    (x, y, w, h): (usize, usize, usize, usize),
    grid: &Vec<Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let directions = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        let ey = if *dy == -1 { 0 } else { h - 1 };
        let ex = if *dx == -1 { 0 } else { w - 1 };
        if nx >= 0 && nx < grid.len() as isize && ny >= 0 && ny < grid[0].len() as isize {
            let nx = nx as usize;
            let ny = ny as usize;
            let mut can_move = true;
            for i in 0..if *dx == 0 { h } else { w } {
                if *dx == 0 {
                    if nx + i >= grid[0].len()
                        || ny + ey >= grid.len()
                        || !can_pass(grid[nx + i][ny + ey])
                    {
                        can_move = false;
                        break;
                    }
                } else {
                    if ny + i >= grid.len()
                        || nx + ex >= grid[0].len()
                        || !can_pass(grid[nx + ex][ny + i])
                    {
                        can_move = false;
                        break;
                    }
                }
            }
            if can_move {
                neighbors.push((nx, ny));
            }
        }
    }
    neighbors
}

pub fn a_star(
    grid: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut dist = vec![vec![None; grid[0].len()]; grid.len()];
    let mut heap = BinaryHeap::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    dist[start.0][start.1] = Some(0);
    heap.push((start.0, start.1, 0));

    while let Some(node) = heap.pop() {
        let (x, y, cost) = node;
        if (x, y) == goal {
            let mut path = Vec::new();
            let mut current = (x, y);
            while current != start {
                path.push(current);
                current = *parent.get(&current).unwrap();
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        if cost > dist[x][y].unwrap() {
            continue;
        }

        for &(next_x, next_y) in &get_neighbors((x, y, 2, 2), grid) {
            let f = cost + 1 + heuristic((next_x, next_y), goal);
            if dist[next_x][next_y].is_none() || f < dist[next_x][next_y].unwrap() {
                heap.push((next_x, next_y, f));
                dist[next_x][next_y] = Some(f);
                parent.insert((next_x, next_y), (x, y));
            }
        }
    }
    None
}

pub fn a_star_len(
    grid: &Vec<Vec<usize>>,
    start: (usize, usize),
    path_len: usize,
) -> Option<Vec<(usize, usize)>> {
    let mut dist = vec![vec![None; grid[0].len()]; grid.len()];
    let mut heap = BinaryHeap::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    dist[start.0][start.1] = Some(0);
    heap.push((start.0, start.1, 0));
    while let Some(node) = heap.pop() {
        let (x, y, len) = node;
        if len == path_len {
            let mut path = Vec::new();
            let mut current = (x, y);
            while current != start {
                path.push(current);
                current = *parent.get(&current).unwrap();
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }
        for &(next_x, next_y) in &get_neighbors((x, y, 2, 2), grid) {
            let f = len + 1;
            if dist[next_x][next_y].is_none() || f < dist[next_x][next_y].unwrap() {
                heap.push((next_x, next_y, f));
                dist[next_x][next_y] = Some(f);
                parent.insert((next_x, next_y), (x, y));
            }
        }
    }
    None
}

/* ---------------------- */
