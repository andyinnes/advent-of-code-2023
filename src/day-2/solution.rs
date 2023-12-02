use std::collections::HashMap;

fn id_from_game(game: &str) -> i32 {
    let str_id = game.replace("Game ", "");
    str_id.parse::<i32>().unwrap()
}

fn game_fails<'a>(game: &'a str, limits: &HashMap<&str, i32>, max_counter: &mut HashMap<&'a str, i32>) -> bool {
    let split = game.split(", ");
    let mut limits_breached = false;

    for colour_count in split {
        let trimmed = colour_count.trim();
        let mut inner_split = trimmed.split(" ");
        let count = inner_split.next().expect("").parse::<i32>().unwrap();
        let colour = inner_split.next().expect("");
        if count > max_counter[colour] {
            max_counter.insert(colour, count);
        }
        limits_breached |= count > limits[colour];
    }
    limits_breached
}

fn solve_games(game_row: &str, limits: &HashMap<&str, i32>) -> (bool, i32) {
    let split = game_row.split(";");
    let mut max_values = HashMap::from([
        ("red", 1),
        ("green", 1),
        ("blue", 1)
    ]);
    let mut too_few_balls = false;
    for game in split {
        too_few_balls |= game_fails(game, limits, &mut max_values);
    }
    let power = max_values.into_values().product();
    (too_few_balls, power)
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let problem_1_limits: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    let mut result_1 = 0;
    let mut result_2 = 0;
    for input_line in contents.lines() {
        let mut split = input_line.split(":");
        let game_id = id_from_game(split.next().expect(""));
        let game_row = split.next().expect("");
        let game_stats = solve_games(game_row, &problem_1_limits);
        if !game_stats.0 {
            result_1 += game_id;
        }
        result_2 += game_stats.1;
    }
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}
