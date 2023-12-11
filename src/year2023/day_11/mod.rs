use std::collections::HashSet;
use itertools::Itertools;

enum SpaceType {
    Galaxy(i32),
    Empty,
    Expanded,
}

fn parse(contents: &str) -> Vec<Vec<Option<i32>>> {
    let mut output: Vec<Vec<Option<i32>>> = vec![];
    let mut galaxy_count = 0;
    let base_col_count = contents.lines().next().unwrap().len();
    let mut columns_without_galaxy: HashSet<usize> = HashSet::from_iter(0..base_col_count);
    for line in contents.lines() {
        let mut row: Vec<Option<i32>> = vec![];
        let mut null_row = true;
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                row.push(None);
            } else {
                null_row = false;
                row.push(Some(galaxy_count));
                galaxy_count += 1;
                columns_without_galaxy.remove(&col);
            }
        }
        output.push(row.clone());
        if null_row {
            output.push(row);
        }
    }
    if columns_without_galaxy.len() > 0 {
        for col in columns_without_galaxy.iter().sorted().rev() {
            for row in output.iter_mut() {
                row.insert(*col, None);
            }
        }
    }
    output
}

fn galaxies(map: &Vec<Vec<Option<i32>>>) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col != None {
                output.push((r, c));
            }
        }
    }
    output
}

fn path_length(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let mut dist = 0;
    if a.0 > b.0 {
        dist += a.0 - b.0;
    } else {
        dist += b.0 - a.0;
    }
    if a.1 > b.1 {
        dist += a.1 - b.1;
    } else {
        dist += b.1 - a.1;
    }
    dist
}

fn problem_1(galaxy_list: &Vec<(usize, usize)>) -> usize {
    let mut result = 0;
    for i in 0..galaxy_list.len() - 1 {
        let left = galaxy_list[i];
        for j in i + 1..galaxy_list.len() {
            let right = galaxy_list[j];
            result += path_length(&left, &right);
        }
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let readings = parse(contents);
    let galaxy_list = galaxies(&readings);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&galaxy_list), 2)
}