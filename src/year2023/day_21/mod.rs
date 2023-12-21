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

fn to_base_map(map: &Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let mut output_map = vec![];
    for y in 0..map.len() {
        let mut current_row = vec![];
        for x in 0..map[0].len() {
            current_row.push(match &map[y][x] {
                Plot::Start => Plot::Garden,
                a => a.clone(),
            });
        }
        output_map.push(current_row);
    }
    output_map
}

fn curve_fit(known: &Vec<(i128, i128)>, x: i128) -> i128 {
    let a = known[0].1 * ((x - known[1].0) * (x - known[2].0)) / ((known[0].0 - known[1].0) * (known[0].0 - known[2].0));
    let b = known[1].1 * ((x - known[0].0) * (x - known[2].0)) / ((known[1].0 - known[0].0) * (known[1].0 - known[2].0));
    let c = known[2].1 * ((x - known[0].0) * (x - known[1].0)) / ((known[2].0 - known[0].0) * (known[2].0 - known[1].0));
    a + b + c
}

fn problem_2(map: &Vec<Vec<Plot>>) -> i128 {
    let base_map = to_base_map(map);
    let mut stepped = map.clone();
    let mut outputs: Vec<(i128, i128)> = vec![];
    for step in 0..1000 {
        stepped = do_step(&stepped.clone());
        let mut edge_reached = false;
        if stepped[0].iter().any(|item| *item == Plot::Visited) {
            // Add the grids above
            edge_reached = true;
            let repeats = stepped[0].len();
            for y in (0..base_map.len()).rev() {
                let row: Vec<Plot> = base_map[y].iter().cycle().take(repeats).map(|x| x.clone()).collect();
                stepped.insert(0, row);
            }
        }
        if stepped[stepped.len() - 1].iter().any(|item| *item == Plot::Visited) {
            // Add the grids below
            edge_reached = true;
            let repeats = stepped[0].len();
            for y in 0..base_map.len() {
                let row: Vec<Plot> = base_map[y].iter().cycle().take(repeats).map(|x| x.clone()).collect();
                stepped.push(row);
            }
        }
        if (0..stepped.len()).any(|i| stepped[i][0] == Plot::Visited) {
            // Add grids to the left
            edge_reached = true;
            let count = stepped.len().clone();
            for y in 0..count {
                let current = stepped[y].clone();
                stepped[y] = base_map[y % base_map.len()].clone();
                stepped[y].extend(current.clone());
            }
        }
        if (0..stepped.len()).any(|i| stepped[i][stepped[0].len() - 1] == Plot::Visited) {
            // Add grids to the right
            edge_reached = true;
            let count = stepped.len().clone();
            for y in 0..count {
                stepped[y].extend(
                    base_map[y % base_map.len()].clone()
                );
            }
        }
        if edge_reached {
            outputs.push((step + 1, stepped.iter().map(|line| line.iter().filter(|item| **item == Plot::Visited).count()).sum::<usize>() as i128));
        }
        if outputs.len() >= 3 {
            break;
        }
    }
    println!("{:?}", outputs);
    curve_fit(&outputs, 26501365)
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let map = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&map), problem_2(&map))
}