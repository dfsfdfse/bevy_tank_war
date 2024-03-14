use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::Read,
};

use serde::Deserialize;
use serde_ron::de::from_bytes;
fn main() {
    let map = load_ron();
    let start = (4, 1);
    let goal = (25, 1);
    let time_start = std::time::Instant::now();
    let path = a_star(&map.maps[0].map, start, goal);
    println!("{:?}", path);
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

fn get_neighbors((x, y): (usize, usize), grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 && can_pass(grid[x - 1][y]) {
        neighbors.push((x - 1, y));
    }
    if y > 0 && can_pass(grid[x][y - 1]) {
        neighbors.push((x, y - 1));
    }
    if x < grid.len() - 1 && can_pass(grid[x + 1][y]) {
        neighbors.push((x + 1, y));
    }
    if y < grid[0].len() - 1 && can_pass(grid[x][y + 1]) {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn move_up() -> bool{
    true
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

        for &(next_x, next_y) in &get_neighbors((x, y), grid) {
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



fn tank_a_star(grid: &Vec<Vec<usize>>, start: &Vec<(usize, usize)>, goal: &Vec<(usize, usize)>) {}
