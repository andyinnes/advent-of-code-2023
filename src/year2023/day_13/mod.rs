use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Floor {
    Ash,
    Rocks,
}

impl From<char> for Floor {
    fn from(value: char) -> Self {
        match value {
            '.' => Floor::Ash,
            '#' => Floor::Rocks,
            _ => panic!("")
        }
    }
}

type VolcanoMap = Vec<Vec<Floor>>;

fn check_equality(map: &VolcanoMap, a: usize, b: usize, horizontal: bool, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
    if !cache.contains_key(&(a, b)) {
        let mut diff_count = 0;
        if horizontal {
            for x in 0..map[0].len() {
                if map[a][x] != map[b][x] {
                    diff_count += 1;
                }
            }
        } else {
            for y in 0..map.len() {
                if map[y][a] != map[y][b] {
                    diff_count += 1;
                }
            }
        }
        cache.insert((a, b), diff_count);
    }
    cache[&(a, b)]
}

fn find_reflection(map: &VolcanoMap, horizontal: bool, expected_diffs: i32) -> Option<usize> {
    let mut cache = HashMap::new();
    let max_iter = if horizontal {
        map.len() - 1
    } else {
        map[0].len() - 1
    };
    for start in 0..max_iter {
        let mut diff_count = 0;
        if check_equality(map, start, start + 1, horizontal, &mut cache) <= expected_diffs {
            diff_count += check_equality(map, start, start + 1, horizontal, &mut cache);
            let row_count = start.min(max_iter - 1 - start);
            for delta in 1..=row_count {
                diff_count += check_equality(map, start - delta, start + 1 + delta, horizontal, &mut cache);
                if diff_count > expected_diffs {
                    break;
                }
            }
            if diff_count == expected_diffs {
                return Some(start + 1);
            }
        }
    }
    None
}

fn score_map(map: &VolcanoMap, expected_diffs: i32) -> usize {
    let result_h = find_reflection(map, true, expected_diffs);
    let output = if result_h == None {
        find_reflection(map, false, expected_diffs).unwrap()
    } else {
        result_h.unwrap() * 100
    };
    output
}

fn parse(contents: &str) -> Vec<VolcanoMap> {
    let mut map_vec = vec![];
    let mut current_map: VolcanoMap = vec![];
    for line in contents.lines() {
        if line == "" {
            map_vec.push(current_map.clone());
            current_map = vec![];
        } else {
            current_map.push(line.chars().map(Floor::from).collect());
        }
    }
    map_vec.push(current_map);
    map_vec

}


pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let data = parse(contents);
    let problem_1: usize = data.iter().map(|x| score_map(x, 0)).sum();
    let problem_2: usize = data.iter().map(|x| score_map(x, 1)).sum();
    format!("Problem 1: {}\nProblem 2: {}", problem_1, problem_2)
}