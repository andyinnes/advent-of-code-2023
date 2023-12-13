use std::collections::HashSet;

#[derive(Debug)]
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
            Direction::Up => Coord { y: self.y + step.count, ..*self },
            Direction::Down => Coord { y: self.y - step.count, ..*self },
            Direction::Left => Coord { x: self.x - step.count, ..*self },
            Direction::Right => Coord { x: self.x + step.count, ..*self },
        }
    }

    fn follow(&self, tail: &Self) -> Vec<Self> {
        // Follow the head and return a list of visited nodes
        let mut visited = vec![];
        let delta = (self.x - tail.x, self.y - tail.y);
        let delta_abs = (delta.0.abs(), delta.1.abs());
        if delta_abs.0.max(delta_abs.1) == 1 {
            return vec![];
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

#[derive(Clone, Copy)]
struct Status {
    head: Coord,
    tail: Coord,
}

impl Status {
    fn apply_step(&mut self, step: &Step, visited: &mut HashSet<Coord>) -> () {
        let new_head = self.head.apply_step(&step);
        let newly_visited = new_head.follow(&self.tail);
        for point in &newly_visited {
            visited.insert(*point);
        }
        self.head = new_head;
        if newly_visited.len() > 0 {
            self.tail = newly_visited[newly_visited.len() - 1];
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

fn print(status: &Status) -> () {
    let start = Coord {x: 0, y: 0};
    for y in 0..5 {
        for x in 0..6 {
            let c = Coord { x, y: 4 - y };
            if c == status.head {
                print!("H");
            } else if c == status.tail {
                print!("T");
            } else if c == start {
                print!("s");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("");
}

fn visits(status: &HashSet<Coord>) -> () {
    for y in 0..5 {
        for x in 0..6 {
            let c = Coord { x, y: 4 - y };
            if status.contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("");
}

fn problem_1(steps: &Vec<Step>) -> usize {
    let mut status = Status {
        head: Coord { x: 0, y: 0 },
        tail: Coord { x: 0, y: 0 },
    };
    let mut visited: HashSet<Coord> = HashSet::from([status.tail]);
    print(&status);
    for step in steps {
        // println!("{:?} -> {}", step.direction, step.count);
        status.apply_step(&step, &mut visited);
        // println!("{:?}", visited);
        // print(&status);
    }
    visits(&visited);
    visited.len()
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let data = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&data), 2)
}