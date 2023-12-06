// Implementing a better solution, took a rough route because rust is new to me.
use std::{cmp, collections::HashMap};

#[derive(Clone, Copy)]
struct PartNum {
    value: i32,
    row: usize,
    start: usize,
    end: usize,
}

impl PartNum {
    fn new(value: String, row: usize, start: usize, end: usize) -> PartNum {
        PartNum { value: value.parse::<i32>().unwrap(), row, start, end }
    }
}

fn read_data(file: &str) -> Option<Vec<PartNum>> {
    let mut output: Vec<PartNum> = Vec::new();
    for (row, line) in file.lines().enumerate() {
        let mut current_num = String::from("");
        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_num.push(c);
            } else if current_num != "" {
                let start_pos = col - current_num.len();
                let current_part = PartNum::new(current_num, row, start_pos, col - 1);
                output.push(current_part);
                current_num = String::from("");
            }
        }
        if current_num != "" {
            let max = line.len();
            let start_pos = max - current_num.len();
            let current_part = PartNum::new(current_num, row, start_pos, max - 1);
            output.push(current_part);
        }
    }
    Some(output)
}

fn adjacent_points(lines: Vec<&str>, part_numbers: &Vec<PartNum>) -> (i32, i32) {
    let mut output = Vec::new();
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let max_col = lines[0].len();
    let max_row = lines.len();
    for part_number in part_numbers {
        let mut include: bool = false;
        let mut include_gear: Vec<(usize, usize)> = Vec::new();
        let row_start = if part_number.row == 0 {0} else {part_number.row - 1};
        for row in row_start..cmp::min(max_row, part_number.row + 2) {
            let col_start = if part_number.start == 0 {0} else {part_number.start - 1};
            for (col_mod, c) in lines[row][col_start..cmp::min(max_col, part_number.end + 2)].chars().enumerate() {
                if c != '.' && !c.is_numeric() {
                    include = true;
                    if c == '*' {
                        include_gear.push((row, col_start + col_mod));
                    }
                }
            }
        }
        if include {
            output.push(part_number.value);
        }
        for gear in include_gear {
            gears.entry(gear).and_modify(|x| x.push(part_number.value)).or_insert(vec![part_number.value]);
        }
    }
    let result_1 = output.iter().sum();
    let mut result_2 = 0;
    for part_list in gears.values() {
        if part_list.len() == 2 {
            result_2 += part_list.iter().product::<i32>();
        }
    }
    (result_1, result_2)
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let part_numbers = read_data(contents).unwrap();
    let (res1, res2) = adjacent_points(contents.lines().collect(), &part_numbers);
    format!("Problem 1: {res1}\nProblem 2: {res2}")
}
