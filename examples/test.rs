use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::Read,
};

use serde::Deserialize;
use serde_ron::de::from_bytes;
fn main() {
    let map = load_ron();
    let start = (0, 0);
    let goal = (23, 0);
    let time_start = std::time::Instant::now();
    let path = a_star(&map.maps[0].map, start, goal);
    if let Some(path) = path {
        println!("{:?}", path);
    }
    println!("Time: {}ms", time_start.elapsed().as_millis());
}

#[derive(Deserialize)]
struct Maps {
    pub maps: Vec<Map>,
}

#[derive(Deserialize)]
struct Map {
    pub map: Vec<Vec<usize>>,
    //pub name: String,
}
fn load_ron() -> Maps {
    let mut file = File::open("assets/map.ron").expect("open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    from_bytes(contents.as_bytes()).unwrap()
}

fn can_pass(blk_type: usize) -> bool {
    [0, 3, 4].contains(&blk_type)
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

fn a_star(
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

        for &(next_x, next_y) in &get_neighbors((x, y, 3, 3), grid) {
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
