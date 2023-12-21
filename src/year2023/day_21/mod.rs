#[derive(Clone, PartialEq, Eq)]
enum Plot {
    Rocks,
    Garden,
    Start,
    Visited,
}

impl From<char> for Plot {
    fn from(value: char) -> Self {
        match value {
            '#' => Plot::Rocks,
            '.' => Plot::Garden,
            'S' => Plot::Start,
            _ => panic!("Unexpected token"),
        }
    }
}

// fn print_map(map: &Vec<Vec<Plot>>) -> () {
//     for y in 0..map.len() {
//         for x in 0..map[0].len() {
//             print!("{}", match map[y][x] {
//                 Plot::Garden => ".",
//                 Plot::Start => "S",
//                 Plot::Rocks => "#",
//                 Plot::Visited => "O",
//             });
//         }
//         print!("\n");
//     }
// }

fn parse(contents: &str) -> Vec<Vec<Plot>> {
    contents.lines().map(|line| line.chars().map(Plot::from).collect()).collect()
}

fn can_visit(x: usize, y: usize, map: &Vec<Vec<Plot>>) -> Plot {
    match map[y][x] {
        Plot::Rocks => {return Plot::Rocks;},
        _ => (),
    }
    if x > 0 {
        match map[y][x - 1] {
            Plot::Visited | Plot::Start => {return Plot::Visited;},
            _ => (),
        }
    }
    if y > 0 {
        match map[y - 1][x] {
            Plot::Visited | Plot::Start => {return Plot::Visited;},
            _ => (),
        }
    }
    if x < map[0].len() - 1 {
        match map[y][x + 1] {
            Plot::Visited | Plot::Start => {return Plot::Visited;},
            _ => (),
        }
    }
    if y < map.len() - 1 {
        match map[y + 1][x] {
            Plot::Visited | Plot::Start => {return Plot::Visited;},
            _ => (),
        }
    }
    Plot::Garden
}

fn do_step(map: &Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let mut output_map = vec![];
    for y in 0..map.len() {
        let mut current_row = vec![];
        for x in 0..map[0].len() {
            current_row.push(can_visit(x, y, map));
        }
        output_map.push(current_row);
    }
    output_map
}

fn problem_1(map: &Vec<Vec<Plot>>) -> usize {
    let mut stepped = map.clone();
    for _ in 0..64 {
        stepped = do_step(&stepped).clone();
    }
    stepped.iter().map(|line| line.iter().filter(|item| **item == Plot::Visited).count()).sum()
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let map = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&map), 2)
}