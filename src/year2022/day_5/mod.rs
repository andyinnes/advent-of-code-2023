use std::collections::HashMap;

fn solver(contents: &str, window: usize) -> usize {
    let mut char_map = HashMap::new();
    let char_vec: Vec<char> = contents.chars().collect();
    for i in 0..window {
        let c = char_vec[i];
        if char_map.contains_key(&c) {
            char_map.insert(c, char_map[&c] + 1);
        } else {
            char_map.insert(c, 1);
        }
    }
    if char_map.len() == window {
        return window + 1;
    }
    for i in window..char_vec.len() {
        let prev = char_vec[i - window];
        let current = char_vec[i];
        if char_map.contains_key(&current) {
            char_map.insert(current, char_map[&current] + 1);
        } else {
            char_map.insert(current, 1);
        }
        if char_map[&prev] == 1 {
            char_map.remove(&prev);
        } else {
            char_map.insert(prev, char_map[&prev] - 1);
        }
        if char_map.len() == window {
            return i + 1;
        }
    }
    0
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    format!{"Problem 1 {}\nProblem 2 {}", solver(contents, 4), solver(contents, 14)}
}