#[derive(Clone, PartialEq, Eq)]
struct MapRow {
    elements: Vec<char>,
    counts: Vec<usize>,
}

fn insert_broken_section(len: usize, row_part: &[char], row_counts_part: &[usize], memory: &mut Vec<Vec<Option<usize>>>) -> Option<usize> {
    // println!("Insert {len}, remaining {:?}, {:?}", row_part, row_counts_part);
    if len > row_part.len() {
        Some(0)
    } else if row_part[..len].iter().any(|x| *x == '.') {
        Some(0)
    } else if len == row_part.len() {
        arrangements(&row_part[len..], &row_counts_part[1..], memory)
    } else if row_part[len] == '#' {
        Some(0)
    } else {
        arrangements(&row_part[len + 1..], &row_counts_part[1..], memory)
    }
}

fn arrangements(row_part: &[char], row_counts_part: &[usize], memory: &mut Vec<Vec<Option<usize>>>) -> Option<usize> {
    if let memo @ Some(_) = memory[row_part.len()][row_counts_part.len()] {
        return memo;
    } 
    let outcome = match (row_part.iter().next(), row_counts_part.iter().next()) {
        (Some('.'), _) => arrangements(&row_part[1..], row_counts_part, memory),
        (Some('#'), None) => Some(0),
        (Some('#'), Some(len)) => insert_broken_section(*len, row_part, row_counts_part, memory),
        (Some('?'), None) => arrangements(&row_part[1..], row_counts_part, memory),
        (Some('?'), Some(len)) => {
            let include = insert_broken_section(*len, row_part, row_counts_part, memory);
            let exclude = arrangements(&row_part[1..], row_counts_part, memory);
            let outcome = include.unwrap_or(0) + exclude.unwrap_or(0);
            if outcome > 0 {
                Some(outcome)
            } else {
                Some(0)
            }
        }
        (None, Some(_)) => Some(0),
        (None, None) => Some(1),
        (Some(_), _) => panic!("Invalid char"),
    };
    memory[row_part.len()][row_counts_part.len()] = outcome;
    outcome
}

fn arragements_memoized(row: &Vec<char>, row_counts: &Vec<usize>) -> usize {
    let mut memory: Vec<Vec<Option<usize>>> = vec![vec![None; row_counts.len() + 1]; row.len() + 1];
    arrangements(row, row_counts, &mut memory).unwrap()
}

fn parse(contents: &str) -> Vec<MapRow> {
    let mut output = vec![];
    for line in contents.lines() {
        let (map, values) = line.split_once(' ').unwrap();
        let map_vec = map.chars().collect();
        let counts = values.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        output.push(MapRow { elements: map_vec, counts });
    }
    output
}

fn problem_1(data: &Vec<MapRow>) -> usize {
    let mut output = 0;
    for row in data {
        output += arragements_memoized(&row.elements, &row.counts);
    }
    output
}

fn problem_2(data: &Vec<MapRow>) -> usize {
    let mut output = 0;
    for row in data.iter() {
        let mut mult_row = row.elements.clone();
        let mut mult_counts = row.counts.clone();
        for _ in 0..4 {
            mult_row.push('?');
            mult_row.extend(row.elements.clone());
            mult_counts.extend(row.counts.clone());
        }
        output += arragements_memoized(&mult_row, &mult_counts);
    }
    output
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let data = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&data), problem_2(&data))
}