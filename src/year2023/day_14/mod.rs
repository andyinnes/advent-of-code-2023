use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rocks {
    Round,
    Square,
    Empty,
}

impl From<char> for Rocks {
    fn from(value: char) -> Self {
        match value {
            '.' => Rocks::Empty,
            '#' => Rocks::Square,
            'O' => Rocks::Round,
            _ => panic!("")
        }
    }
}

type RockMap = Vec<Vec<Rocks>>;

fn map_to_string(map: &RockMap) -> String {
    let mut output = String::new();
    for row in map {
        for val in row {
            output += match val {
                Rocks::Square => "#",
                Rocks::Round => "O",
                _ => ".",
            }
        }
        output += "\n";
    }
    output
}

fn tip_rocks_north(map: &mut RockMap) -> () {
    let row_count = map.len();
    let col_count = map[0].len();
    for col in 0..col_count {
        let mut min_row = 0;
        for row in 0..row_count {
            let current = map[row][col];
            if current == Rocks::Round {
                if min_row != row {
                    map[min_row][col] = Rocks::Round;
                    map[row][col] = Rocks::Empty;
                }
                min_row += 1;
            } else if current == Rocks::Square {
                min_row = row + 1;
            }
        }
    }
}
fn tip_rocks_south(map: &mut RockMap) -> () {
    let row_count = map.len();
    let col_count = map[0].len();
    for col in (0..col_count).rev() {
        let mut max_row = row_count - 1;
        for row in (0..row_count).rev() {
            let current = map[row][col];
            if current == Rocks::Round {
                if max_row != row {
                    map[max_row][col] = Rocks::Round;
                    map[row][col] = Rocks::Empty;
                }
                if max_row != 0 {
                    max_row -= 1;
                }
            } else if current == Rocks::Square && row != 0 {
                max_row = row - 1;
            }
        }
    }
}
fn tip_rocks_west(map: &mut RockMap) -> () {
    let row_count = map.len();
    let col_count = map[0].len();
    for row in 0..row_count {
        let mut min_col = 0;
        for col in 0..col_count {
            let current = map[row][col];
            if current == Rocks::Round {
                if min_col != col {
                    map[row][min_col] = Rocks::Round;
                    map[row][col] = Rocks::Empty;
                }
                min_col += 1;
            } else if current == Rocks::Square {
                min_col = col + 1;
            }
        }
    }
}
fn tip_rocks_east(map: &mut RockMap) -> () {
    let row_count = map.len();
    let col_count = map[0].len();
    for row in (0..row_count).rev() {
        let mut max_col = col_count - 1;
        for col in (0..col_count).rev() {
            let current = map[row][col];
            if current == Rocks::Round {
                if max_col != col {
                    map[row][max_col] = Rocks::Round;
                    map[row][col] = Rocks::Empty;
                }
                if max_col != 0 {
                    max_col -= 1;
                }
            } else if current == Rocks::Square && col != 0 {
                max_col = col - 1;
            }
        }
    }
}

fn spin_cycle(map: &mut RockMap) -> () {
    tip_rocks_north(map);
    tip_rocks_west(map);
    tip_rocks_south(map);
    tip_rocks_east(map);
}

fn score_map(map: &RockMap) -> usize {
    let row_count = map.len();
    let col_count = map[0].len();
    let mut score = 0;
    for x in 0..col_count {
        for y in 0..row_count {
            if map[y][x] == Rocks::Round {
                score += row_count - y;
            }
        }
    }
    score
}

fn parse(contents: &str) -> RockMap {
    contents.lines().into_iter().map(|line| line.chars().map(Rocks::from).collect()).collect()
}

fn problem_2(data: &mut RockMap) -> usize {
    let loop_size = 1_000_000_000;
    let mut cycle_vec = vec![];
    let mut cycle_set = HashSet::new();
    let start_str = map_to_string(&data);
    cycle_set.insert(start_str.clone());
    cycle_vec.push(start_str);
    let mut current_str = "".to_string();
    for _ in 0..loop_size {
        spin_cycle(data);
        current_str = map_to_string(&data);
        if cycle_set.contains(&current_str) {
            break;
        }
        cycle_set.insert(current_str.clone());
        cycle_vec.push(current_str.clone());
    }
    let first_occurrence = cycle_vec.iter().position(|x| *x == current_str).unwrap();
    let next_occurrence = cycle_vec.len();
    let remaining_steps = (loop_size - first_occurrence) % (next_occurrence - first_occurrence);
    println!("{}, {}, {}", first_occurrence, next_occurrence, remaining_steps);
    for _ in 0..remaining_steps {
        spin_cycle(data);
    }
    score_map(&data)
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut data = parse(contents);
    tip_rocks_north(&mut data);
    let mut data2 = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", score_map(&data), problem_2(&mut data2))
}