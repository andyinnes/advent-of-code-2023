use std::collections::HashSet;

#[derive(Clone, Copy)]
enum TileType {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    ForwardMirror,
    BackMirror,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '.' => TileType::Empty,
            '|' => TileType::VerticalSplitter,
            '-' => TileType::HorizontalSplitter,
            '/' => TileType::ForwardMirror,
            '\\' => TileType::BackMirror,
            _ => panic!("")
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn apply_direction(x: usize, y: usize, direction: &Direction, visited: &HashSet<(usize, usize, Direction)>) -> Option<(usize, usize)> {
    let (mut new_x, mut new_y) = (x, y);
    if direction == &Direction::Up {
        if y == 0 {
            return None;
        }
        new_y = y - 1;
    } else if direction == &Direction::Down {
        new_y = y + 1;
    } else if direction == &Direction::Left {
        if x == 0 {
            return None;
        }
        new_x = x - 1;
    } else {
        new_x = x + 1;
    }
    let coord = (new_x, new_y, direction.clone());
    if visited.contains(&coord) {
        return None;
    }
    Some((new_x, new_y))
}

fn new_directions(tile: &TileType, current_direction: &Direction) -> Vec<Direction> {
    let mut output = vec![];
    match tile {
        TileType::Empty => output.push(current_direction.clone()),
        TileType::ForwardMirror => {
            match current_direction {
                Direction::Up => output.push(Direction::Right),
                Direction::Down => output.push(Direction::Left),
                Direction::Left => output.push(Direction::Down),
                Direction::Right => output.push(Direction::Up),
            }
        },
        TileType::BackMirror => {
            match current_direction {
                Direction::Up => output.push(Direction::Left),
                Direction::Down => output.push(Direction::Right),
                Direction::Left => output.push(Direction::Up),
                Direction::Right => output.push(Direction::Down),
            }
        },
        TileType::HorizontalSplitter =>{
            match current_direction {
                Direction::Down | Direction::Up => {
                    output.push(Direction::Right);
                    output.push(Direction::Left);
                },
                _ => {
                    output.push(current_direction.clone());
                }
            }
        },
        TileType::VerticalSplitter =>{
            match current_direction {
                Direction::Left | Direction::Right => {
                    output.push(Direction::Up);
                    output.push(Direction::Down);
                },
                _ => {
                    output.push(current_direction.clone());
                }
            }
        }
    }
    output    
}

fn parse(contents: &str) -> Vec<Vec<TileType>> {
    let mut output = vec![];
    for line in contents.lines() {
        output.push(line.chars().map(TileType::from).collect());
    }
    output
}

fn dfs(x: usize, y: usize, in_direction: &Direction, data: &Vec<Vec<TileType>>, visited: &mut HashSet<(usize, usize, Direction)>) -> Option<usize> {
    if visited.contains(&(x, y, in_direction.clone())) {
        return None;
    }
    visited.insert((x, y, in_direction.clone()));
    let x_limit = data[0].len();
    let y_limit = data.len();
    let current_tile = data[y][x];
    let new_directions = new_directions(&current_tile, &in_direction);
    for dir in new_directions {
        let new_location = apply_direction(x, y, &dir, visited);
        if new_location != None {
            let (new_x, new_y) = new_location.unwrap();
            if new_x < x_limit && new_y < y_limit {
                dfs(new_x, new_y, &dir, data, visited);
            }
        }
    }
    None
}

fn solver(start: (usize, usize), start_direction: &Direction, data: &Vec<Vec<TileType>>) -> usize {
    let mut visited = HashSet::new();
    dfs(start.0, start.1, start_direction, data, &mut visited);
    let output: HashSet<(usize, usize)> = visited.into_iter().map(|(x, y, _)| (x, y)).collect();
    output.len()

}

fn problem_1(data: &Vec<Vec<TileType>>) -> usize {
    solver((0, 0), &Direction::Right, data)
}

fn problem_2(data: &Vec<Vec<TileType>>) -> usize {
    let mut max_result = 0;
    for x in 0..data[0].len() {
        max_result = max_result.max(solver((x, 0), &Direction::Down, data));
        max_result = max_result.max(solver((x, data.len() - 1), &Direction::Up, data));
    }
    for y in 0..data.len() {
        max_result = max_result.max(solver((0, y), &Direction::Right, data));
        max_result = max_result.max(solver((data[0].len() - 1, y), &Direction::Left, data));
    }
    max_result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let data = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&data), problem_2(&data))
}