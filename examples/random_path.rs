use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

// 定义方向枚举
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 生成随机路径函数
fn generate_random_path(
    max_length: usize,
    start: (usize, usize),
    map: &Vec<Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();
    let mut path = Vec::new();
    let mut current_position = start;
    let mut visited = HashSet::new();

    path.push(current_position);
    visited.insert(current_position);

    for _ in 1..max_length {
        let available_directions: Vec<Direction> = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter(|&dir| {
            let next_position = match dir {
                Direction::Up => (current_position.0, current_position.1.saturating_sub(1)),
                Direction::Down => (current_position.0, current_position.1 + 1),
                Direction::Left => (current_position.0.saturating_sub(1), current_position.1),
                Direction::Right => (current_position.0 + 1, current_position.1),
            };
            next_position.0 < map.len()
                && next_position.1 < map[0].len()
                && map[next_position.0][next_position.1] == 0
                && !visited.contains(&next_position)
        })
        .collect();

        if available_directions.is_empty() {
            break;
        }

        let chosen_direction = available_directions.choose(&mut rng).unwrap();
        current_position = match chosen_direction {
            Direction::Up => (current_position.0, current_position.1.saturating_sub(1)),
            Direction::Down => (current_position.0, current_position.1 + 1),
            Direction::Left => (current_position.0.saturating_sub(1), current_position.1),
            Direction::Right => (current_position.0 + 1, current_position.1),
        };
        path.push(current_position);
        visited.insert(current_position);
    }

    path
}

fn main() {
    let max_length = 5; // 最大路径长度
    let map: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let start = (0, 0); // 起始坐标

    let path = generate_random_path(max_length, start, &map);

    println!("Generated path:");
    for coordinate in &path {
        println!("({}, {})", coordinate.0, coordinate.1);
    }
}
