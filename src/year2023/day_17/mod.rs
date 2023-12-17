use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
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

fn apply_move(start: &Position, new_move: &Position, map: &Vec<Vec<i32>>) -> Option<Position> {
    let new_pos = start + new_move;
    if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= (map[0].len() as i32) || new_pos.y >= (map.len() as i32) {
        None
    } else {
        Some(new_pos)
    }
}

#[derive(Clone, Debug, Eq)]
struct State {
    position: Position,
    in_direction: usize,
    moves_in_direction: i32,
    heat_loss: i32,
    history: Vec<Position>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.in_direction == other.in_direction && self.moves_in_direction == other.moves_in_direction
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, hash_state: &mut H) {
        self.position.hash(hash_state);
        self.in_direction.hash(hash_state);
        self.moves_in_direction.hash(hash_state);
    }
}

fn parse(contents: &str) -> Vec<Vec<i32>> {
    contents.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect()
}

fn move_state(map: &Vec<Vec<i32>>, pq: &mut PriorityQueue<State, i32>, current_state: &State, direction_shift: usize) -> Option<State> {
    let new_direction = (current_state.in_direction + direction_shift) % 4;
    let new_position_opt = apply_move(&current_state.position, &MOVES[new_direction], map);
    if new_position_opt.is_some() {
        let position = new_position_opt.unwrap();
        let heat_loss = current_state.heat_loss + map[position.y as usize][position.x as usize];
        let moves_in_direction = if direction_shift == 0 {
            current_state.moves_in_direction + 1
        } else {
            0
        };
        let mut history = current_state.history.clone();
        history.push(position);
        pq.push_increase(
            State {
                position,
                in_direction: new_direction,
                moves_in_direction,
                heat_loss,
                history
            },
            -heat_loss,
        );
    }
    None
}

fn problem_1(map: &Vec<Vec<i32>>) -> i32 {
    let mut state_history: HashMap<Position, State> = HashMap::new();
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    let destination = Position { x: (map[0].len() as i32) - 1, y: (map.len() as i32) - 1 };
    let mut state = State {
        position: Position { x: 0, y: 0 },
        in_direction: 0,
        moves_in_direction: 0,
        heat_loss: 0,
        history: vec![]
    };
    pq.push(state.clone(), -state.heat_loss);
    state.in_direction = 2;
    pq.push(state.clone(), -state.heat_loss);
    while !pq.is_empty() {
        state = pq.pop().unwrap().0;
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        state_history.insert(state.position, state.clone());
        if state.position == destination {
            return state.clone().heat_loss;
        }
        // Right
        {
            let new_direction = (state.in_direction + 4 - 1) % 4;
            let new_position_opt =  apply_move(&state.position, &MOVES[new_direction], map);
            if new_position_opt.is_some() {
                let new_position = new_position_opt.unwrap();
                let new_heat_loss = state.heat_loss + map[new_position.y as usize][new_position.x as usize];
                let mut history = state.history.clone();
                history.push(new_position);
                pq.push_increase(
                    State {
                        position: new_position,
                        in_direction: new_direction,
                        moves_in_direction: 0,
                        heat_loss: new_heat_loss,
                        history
                    },
                    -new_heat_loss,
                );
            }
        }
        // Left
        {
            let new_direction = (state.in_direction + 1) % 4;
            let new_position_opt =  apply_move(&state.position, &MOVES[new_direction], map);
            if new_position_opt.is_some() {
                let new_position = new_position_opt.unwrap();
                let new_heat_loss = state.heat_loss + map[new_position.y as usize][new_position.x as usize];
                let mut history = state.history.clone();
                history.push(new_position);
                pq.push_increase(
                    State {
                        position: new_position,
                        in_direction: new_direction,
                        moves_in_direction: 0,
                        heat_loss: new_heat_loss,
                        history,
                    },
                    -new_heat_loss,
                );
            }
        }
        // Straight
        if state.moves_in_direction < 2 {
            let new_position_opt =  apply_move(&state.position, &MOVES[state.in_direction], map);
            if new_position_opt.is_some() {
                let new_position = new_position_opt.unwrap();
                let new_heat_loss = state.heat_loss + map[new_position.y as usize][new_position.x as usize];
                let mut history = state.history.clone();
                history.push(new_position);
                pq.push_increase(
                    State {
                        position: new_position,
                        in_direction: state.in_direction,
                        moves_in_direction: state.moves_in_direction + 1,
                        heat_loss: new_heat_loss,
                        history,
                    },
                    -new_heat_loss,
                );
            }
        }
    }
    // panic!("");
    0
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let map = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&map), 2)
}