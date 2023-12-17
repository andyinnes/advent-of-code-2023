use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Height {
    Start,
    End,
    Mountain(i32),
}

impl From<char> for Height {
    fn from(value: char) -> Self {
        match value {
            'S' => Height::Start,
            'E' => Height::End,
            x => Height::Mountain(x as i32),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn do_move(&self, movement: &Position, map: &Vec<Vec<Height>>) -> Option<Position> {
        let new_pos = Position { x: self.x + movement.x, y: self.y + movement.y };
        if new_pos.x < 0 || new_pos.y < 0 {
            return None;
        } else if new_pos.x >= (map[0].len() as i32) || new_pos.y >= (map.len() as i32) {
            return None;
        }
        let current_height = map[self.y as usize][self.x as usize];
        let next_height = map[new_pos.y as usize][new_pos.x as usize];
        match (current_height, next_height) {
            (Height::Mountain(current), Height::Mountain(next)) => {
                if current < next - 1 {
                    return None;
                }
            },
            (Height::Mountain(current), Height::End) => {
                if current < ('z' as i32) - 1 {
                    return None;
                }
            }
            _ => (),
        }
        Some(new_pos)
    }
}

impl Add for &Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const MOVES: [Position; 4] = [
    Position { x: -1, y: 0 },
    Position { x: 0, y: 1 },
    Position { x: 1, y: 0 },
    Position { x: 0, y: -1 }
];

#[derive(Clone, Debug, Eq)]
struct State {
    position: Position,
    direction: usize,
    steps: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, hash_state: &mut H) {
        self.position.hash(hash_state);
        self.direction.hash(hash_state);
    }
}

fn next_step(current_state: &State, direction_shift: usize, map: &Vec<Vec<Height>>, pq: &mut PriorityQueue<State, i32>) -> () {
    let direction = (current_state.direction + direction_shift) % 4;
    let position_opt = current_state.position.do_move(&MOVES[direction], map);
    if position_opt.is_some() {
        let new_state = State {
            position: position_opt.unwrap(),
            direction,
            steps: current_state.steps + 1,
        };
        pq.push_increase(new_state, -(current_state.steps + 1));
    }
}

fn path_search(map: &Vec<Vec<Height>>, start: &Position, destination: &Position) -> i32 {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut first_loop = true;
    let mut state = State {
        position: *start,
        direction: 0,
        steps: 0,
    };
    pq.push(state.clone(), -state.steps);
    while !pq.is_empty() {
        state = pq.pop().unwrap().0;
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        if state.position == *destination {
            return state.clone().steps;
        }
        // Right
        next_step(&state, 3, map, &mut pq);
        // Left
        next_step(&state, 1, map, &mut pq);
        // Straight
        next_step(&state, 0, map, &mut pq);
        // Back (only if it's the first step)
        if first_loop {
            first_loop = false;
            next_step(&state, 2, map, &mut pq);
        }
    }
    i32::MAX
}

fn parse(contents: &str) -> (Vec<Vec<Height>>, Position, Position) {
    let mut output = vec![];
    let mut start = None;
    let mut end = None;
    for (y, line) in contents.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let value = Height::from(c);
            if value == Height::Start {
                start = Some(Position { x: x as i32, y: y as i32 });
            } else if value == Height::End {
                end = Some(Position { x: x as i32, y: y as i32 });
            }
            row.push(value);
        }
        output.push(row);
    }
    (output, start.unwrap(), end.unwrap())
}

fn available_starts(map: &Vec<Vec<Height>>) -> Vec<Position> {
    let mut output = vec![];
    let a_height = 'a' as i32;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let current = map[y][x];
            match current {
                Height::Mountain(height) => {
                    if height == a_height {
                        output.push(Position {x: x as i32, y: y as i32 });
                    }
                },
                Height::Start => output.push(Position {x: x as i32, y: y as i32 }),
                _ => (),
            }
        }
    }
    output
}

fn problem(map: &Vec<Vec<Height>>, starts: &[Position], destination: &Position) -> i32 {
    let mut result = i32::MAX;
    for start in starts {
        result = result.min(path_search(map, start, destination));
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (map, start, end) = parse(contents);
    let all_starts = available_starts(&map);
    format!("Problem 1: {}\nProblem 2: {}",
        problem(&map, &[start], &end),
        problem(&map, &all_starts, &end),
    )
}
