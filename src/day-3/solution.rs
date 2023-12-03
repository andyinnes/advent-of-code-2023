fn include_number(row: &mut Vec<char>, start: usize, row_length: usize) -> i32 {
    // Given a starting position, identify if it is a number, then find the starting point and all
    // remaining numbers
    if start >= row_length {
        return 0;
    }
    if !row[start].is_digit(10) {
        return 0;
    }
    let mut current_i = start;
    let mut at_start = false;
    while row[current_i].is_digit(10) {
        if current_i == 0 {
            at_start = true;
            break;
        }
        current_i -= 1;
    }
    let mut output = String::from("");
    if !at_start {
        current_i += 1;
    }
    while current_i < row_length && row[current_i].is_digit(10) {
        output.push(row[current_i]);
        row[current_i] = '.';
        current_i += 1;
    }
    return output.parse::<i32>().unwrap();
}

fn row_iter(lines: &mut Vec<Vec<char>>, current_row: usize, max_row: usize) -> i32 {
    if current_row > max_row {
        return 0;
    }
    let mut result = 0;
    // Run through each row, if you find a special character you loop through the neighbours and remove numbers
    let row_length = lines[current_row].len();
    for i in 0..row_length {
        let c: char = lines[current_row][i];
        if !c.is_digit(10) && c != '.' {
            // Defaults for the current row
            if i != 0 {
                result += include_number(&mut lines[current_row], i - 1, row_length);
            }
            if i < row_length - 1 {
                result += include_number(&mut lines[current_row], i + 1, row_length);
            }
            // For row above
            if current_row > 0 {
                result += include_number(&mut lines[current_row - 1], i, row_length);
                if i != 0 {
                    result += include_number(&mut lines[current_row - 1], i - 1, row_length);
                }
                if i < row_length - 1 {
                    result += include_number(&mut lines[current_row - 1], i + 1, row_length);
                }
            }
            // For row below
            if max_row > 0 && current_row < max_row {
                result += include_number(&mut lines[current_row + 1], i, row_length);
                if i != 0 {
                    result += include_number(&mut lines[current_row + 1], i - 1, row_length);
                }
                if i < row_length - 1 {
                    result += include_number(&mut lines[current_row + 1], i + 1, row_length);
                }
            }
            lines[current_row][i] = '.';
        }
    }
    result
}

fn gear_iter(lines: &mut Vec<Vec<char>>, current_row: usize, max_row: usize) -> i32 {
    if current_row > max_row {
        return 0;
    }
    let mut result_out = 0;
    // Run through each row, if you find a special character you loop through the neighbours and remove numbers
    let row_length = lines[current_row].len();
    for i in 0..row_length {
        let mut neighbours: Vec<i32> = vec![];
        let c: char = lines[current_row][i];
        let mut result = 0;
        if c == '*' {
            println!("Looking at row {current_row} col {i}");
            // Defaults for the current row
            if i != 0 {
                result = include_number(&mut lines[current_row], i - 1, row_length);
                if result != 0 {
                    neighbours.push(result);
                }
            }
            if i < row_length - 1 {
                result = include_number(&mut lines[current_row], i + 1, row_length);
                if result != 0 {
                    neighbours.push(result);
                }
            }
            // For row above
            if current_row > 0 {
                result = include_number(&mut lines[current_row - 1], i, row_length);
                if result != 0 {
                    neighbours.push(result);
                }   
                if i != 0 {
                    result = include_number(&mut lines[current_row - 1], i - 1, row_length);
                    if result != 0 {
                        neighbours.push(result);
                    }
                }
                if i < row_length - 1 {
                    result = include_number(&mut lines[current_row - 1], i + 1, row_length);
                    if result != 0 {
                        neighbours.push(result);
                    }
                }
            }
            // For row below
            if max_row > 0 && current_row < max_row {
                result = include_number(&mut lines[current_row + 1], i, row_length);
                if result != 0 {
                    neighbours.push(result);
                }
                if i != 0 {
                    result = include_number(&mut lines[current_row + 1], i - 1, row_length);
                    if result != 0 {
                        neighbours.push(result);
                    }
                }
                if i < row_length - 1 {
                    result = include_number(&mut lines[current_row + 1], i + 1, row_length);
                    if result != 0 {
                        neighbours.push(result);
                    }
                }
            }
            lines[current_row][i] = '.';
        }
        if neighbours.len() == 2 {
            result_out += neighbours.iter().product::<i32>();
        }
    }
    return result_out;
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut lines: Vec<Vec<char>> = contents.split("\n")
        .map(|s: &str| s.to_string().chars().collect())
        .collect();
    let mut lines2 = lines.to_owned();
    let row_length = lines.len();
    let max_row = row_length - 1;

    let mut result = 0;
    for current_row in 0..row_length {
        result += row_iter(&mut lines, current_row, max_row);
    }
    let mut result_2 = 0;
    for current_row in 0..row_length {
        result_2 += gear_iter(&mut lines2, current_row, max_row)
    }
    format!("Problem 1: {result}\nProblem 2: {result_2}")
}