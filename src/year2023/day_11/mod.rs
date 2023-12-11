use std::collections::HashSet;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq)]
enum SpaceType {
    Galaxy(i32),
    Empty,
    Expanded,
}

fn parse(contents: &str) -> Vec<Vec<SpaceType>> {
    let mut output: Vec<Vec<SpaceType>> = vec![];
    let mut galaxy_count = 0;
    let base_col_count = contents.lines().next().unwrap().len();
    let mut columns_without_galaxy: HashSet<usize> = HashSet::from_iter(0..base_col_count);
    for line in contents.lines() {
        let mut row: Vec<SpaceType> = vec![];
        let mut null_row = true;
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                row.push(SpaceType::Empty);
            } else {
                null_row = false;
                row.push(SpaceType::Galaxy(galaxy_count));
                galaxy_count += 1;
                columns_without_galaxy.remove(&col);
            }
        }
        if null_row {
            output.push(vec![SpaceType::Expanded; row.len()]);
        } else {
            output.push(row);
        }
    }
    if columns_without_galaxy.len() > 0 {
        for col in columns_without_galaxy.iter().sorted().rev() {
            for row in output.iter_mut() {
                row[*col] = SpaceType::Expanded;
            }
        }
    }
    output
}

fn galaxies(map: &Vec<Vec<SpaceType>>) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            match col {
                SpaceType::Galaxy(_) => output.push((r, c)),
                _ => continue
            };
        }
    }
    output
}

fn path_length(map: &Vec<Vec<SpaceType>>, expansion: i128, a: &(usize, usize), b: &(usize, usize)) -> i128 {
    let mut dist = 0;
    let start_row = a.0.min(b.0);
    let end_row = a.0.max(b.0);
    let start_col = a.1.min(b.1);
    let end_col = a.1.max(b.1);
    for row_num in start_row..end_row {
        let current = map[row_num][start_col];
        if current == SpaceType::Expanded {
            dist += expansion;
        } else {
            dist += 1;
        }
    }
    for col_num in start_col..end_col {
        let current = map[start_row][col_num];
        if current == SpaceType::Expanded {
            dist += expansion;
        } else {
            dist += 1;
        }
    }
    dist
}

fn problem_solve(map: &Vec<Vec<SpaceType>>, galaxy_list: &Vec<(usize, usize)>, expansion: i128) -> i128 {
    let mut result = 0;
    for i in 0..galaxy_list.len() - 1 {
        let left = galaxy_list[i];
        for j in i + 1..galaxy_list.len() {
            let right = galaxy_list[j];
            result += path_length(map, expansion, &left, &right);
        }
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let map = parse(contents);
    let galaxy_list = galaxies(&map);
    format!(
        "Problem 1: {}\nProblem 2: {}",
        problem_solve(&map, &galaxy_list, 2),
        problem_solve(&map, &galaxy_list, 1000000),
    )
}