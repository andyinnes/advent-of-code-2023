struct Visibility {
    height: i32,
    visible: bool,
}

fn base_visibility(height: i32, visible: bool) -> Visibility {
    Visibility { height, visible }
}

fn parse(contents: &str) -> Vec<Vec<Visibility>> {
    let mut height_map: Vec<Vec<Visibility>> = vec![];
    for line in contents.lines() {
        let mut current: Vec<Visibility> = vec![];
        let mut max_height = -1;
        // Include in output and visible from left
        for c in line.chars() {
            let height = c.to_digit(10).unwrap() as i32;
            let mut visible = false;
            if height > max_height {
                visible = true;
                max_height = height;
            }
            current.push(base_visibility(height, visible));
        }
        // Visible from right
        max_height = -1;
        for vis in current.iter_mut().rev() {
            if vis.height > max_height {
                vis.visible = true;
                max_height = vis.height;
            }
        }
        height_map.push(current);
    }
    let row_length = height_map[0].len();
    let col_length = height_map.len();
    for col in 0..row_length {
        // Visible from top
        let mut max_height = -1;
        for row in 0..col_length {
            if height_map[row][col].height > max_height {
                max_height = height_map[row][col].height;
                height_map[row][col].visible = true;
            }
        }
        // Visible from bottom
        let mut max_height = -1;
        for row in (0..col_length).rev() {
            if height_map[row][col].height > max_height {
                max_height = height_map[row][col].height;
                height_map[row][col].visible = true;
            }
        }
    }
    height_map
}

fn find_max_tree_score(height_map: &Vec<Vec<Visibility>>) -> usize {
    let mut max_score = 0;
    let col_count = height_map[0].len();
    let row_count = height_map.len();
    for row in 1..row_count - 1 {
        for col in 1..col_count - 1 {
            let mut score = 1;
            let vis = &height_map[row][col];
            // Left
            {
                let mut score_part = 0;
                for inner_col in (0..col).rev() {
                    score_part += 1;
                    if height_map[row][inner_col].height >= vis.height {
                        break;
                    }
                }
                score *= score_part;
            }
            if score == 0 {
                continue;
            }
            // Right
            {
                let mut score_part = 0;
                for inner_col in col + 1..col_count {
                    score_part += 1;
                    if height_map[row][inner_col].height >= vis.height {
                        break;
                    }
                }
                score *= score_part;
            }
            if score == 0 {
                continue;
            }
            // Up
            {
                let mut score_part = 0;
                for inner_row in (0..row).rev() {
                    score_part += 1;
                    if height_map[inner_row][col].height >= vis.height {
                        break;
                    }
                }
                score *= score_part;
            }
            if score == 0 {
                continue;
            }
            // Down
            {
                let mut score_part = 0;
                for inner_row in row + 1..row_count {
                    score_part += 1;
                    if height_map[inner_row][col].height >= vis.height {
                        break;
                    }
                }
                score *= score_part;
            }
            max_score = max_score.max(score);
        }
    }
    max_score
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let height_map = parse(contents);
    let result_1: i32 = height_map.iter().map(|x| x.iter().map(|y| y.visible as i32).sum::<i32>()).sum();
    let result_2 = find_max_tree_score(&height_map);
    format!("Problem 1: {}\nProblem 2: {}", result_1, result_2)
}