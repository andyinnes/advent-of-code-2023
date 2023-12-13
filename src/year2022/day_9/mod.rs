use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Direction {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unexpected direction")
        }
    }
}

#[derive(PartialEq, Eq)]
struct Step {
    direction: Direction,
    count: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn apply_step(&self, step: &Step) -> Self {
        match step.direction {
            Direction::Up => Coord { y: self.y + 1, ..*self },
            Direction::Down => Coord { y: self.y - 1, ..*self },
            Direction::Left => Coord { x: self.x - 1, ..*self },
            Direction::Right => Coord { x: self.x + 1, ..*self },
        }
    }

    fn follow(&self, tail: &Self) -> Vec<Coord> {
        // Follow the head and return a list of visited nodes
        let mut visited = vec![];
        let delta = (self.x - tail.x, self.y - tail.y);
        let delta_abs = (delta.0.abs(), delta.1.abs());
        if delta_abs.0.max(delta_abs.1) == 1 {
            return vec![];
        } else if delta == (2, 2) {
            // idk this isn't great
            visited.push(Coord { x: self.x - 1, y: self.y - 1 });
        } else if delta == (-2, 2) {
            visited.push(Coord { x: self.x + 1, y: self.y - 1 });
        } else if delta == (2, -2) {
            visited.push(Coord { x: self.x - 1, y: self.y + 1 });
        } else if delta == (-2, -2) {
            visited.push(Coord { x: self.x + 1, y: self.y + 1 });
        } else if delta.0 == 0 {
            if delta.1 < 0 {
                for y_delta in delta.1 + 1..=-1 {
                    visited.push(Coord { x: tail.x, y: self.y - y_delta });
                }
            } else {
                for y_delta in (1..delta.1).rev() {
                    visited.push(Coord { x: tail.x, y: self.y - y_delta });
                }
            }
        } else if delta.1 == 0 {
            if delta.0 < 0 {
                for x_delta in delta.0 + 1..=-1 {
                    visited.push(Coord { x: self.x - x_delta, y: tail.y });
                }
            } else {
                for x_delta in (1..delta.0).rev() {
                    visited.push(Coord { x: self.x - x_delta, y: tail.y });
                }
            }
        } else if delta_abs.0 == 1 {
            let next = if delta.1 < 0 {
                Coord { x: self.x, y: tail.y - 1 }
            } else {
                Coord { x: self.x, y: tail.y + 1 }
            };
            visited.push(next);
            visited.extend(self.follow(&next));
        } else if delta_abs.1 == 1 {
            let next = if delta.0 < 0 {
                Coord { x: tail.x - 1, y: self.y }
            } else {
                Coord { x: tail.x + 1, y: self.y }
            };
            visited.push(next);
            visited.extend(self.follow(&next));
        } else {
            panic!("Unexpected outcome {:?}, {:?}", self, tail);
        }
        visited
    }
}

trait ApplySteps {
    fn apply_step(&mut self, step: &Step, visited: &mut HashSet<Coord>) -> ();
}

impl ApplySteps for Vec<Coord> {
    fn apply_step(&mut self, step: &Step, visited: &mut HashSet<Coord>) -> () {
        for _ in 0..step.count {
            let substep = Step { direction: step.direction.clone(), count: 1 };
            self[0] = self[0].apply_step(&substep);
            for i in 1..self.len() {
                let new_i = self[i - 1].follow(&self[i]);
                if new_i.len() == 0 {
                    break;
                }
                self[i] = new_i[0];
            }
            visited.insert(self[self.len() - 1]);
        }
    }
}

fn parse(contents: &str) -> Vec<Step> {
    let mut output = vec![];
    for line in contents.lines() {
        let (dir_str, count_str) = line.split_once(' ').unwrap();
        output.push(Step {
            direction: Direction::from(dir_str),
            count: count_str.parse::<i32>().unwrap(),
        })
    }
    output
}

fn problem_loop(steps: &Vec<Step>, tail_length: usize) -> usize {
    let mut status = vec![Coord { x: 0, y: 0 }; tail_length + 1];
    let mut visited: HashSet<Coord> = HashSet::from([status[0]]);
    for step in steps.iter() {
        status.apply_step(&step, &mut visited);
    }
    visited.len()
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let data = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_loop(&data, 1), problem_loop(&data, 9))
}