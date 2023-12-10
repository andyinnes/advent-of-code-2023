use std::collections::HashMap;

fn parse(contents: &str) -> Vec<(i32, i32)> {
    let mut output = vec![];
    let opponent_lookup = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let player_lookup = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    for line in contents.lines() {
        let row: Vec<&str> = line.split_whitespace().collect();
        if row.len() != 2 {
            panic!("Row is wrong");
        }
        output.push((
            opponent_lookup[row[0]],
            player_lookup[row[1]],
        ));
    }
    output
}

fn problem_1(games: &Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    for game in games {
        // Base score for type played
        result += game.1;
        // addition for outcome
        if game.0 == game.1 {
            result += 3;
        } else if game.0 == game.1 - 1 || (game.1 == 1 && game.0 == 3) {
            result += 6;
        }
    }
    result
}

fn problem_2(games: &Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    let base_score_map = HashMap::from([(1, 0), (2, 3), (3, 6)]);
    for game in games {
        // Base score for outcome
        result += base_score_map[&game.1];
        // additional score for result
        if game.1 == 1 {
            if game.0 == 1 {
                result += 3;
            } else {
                result += game.0 - 1;
            }
        } else if game.1 == 2 {
            result += game.0
        } else {
            if game.0 == 3 {
                result += 1;
            } else {
                result += game.0 + 1;
            }
        }
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let games = parse(contents);
    format!{"Problem 1 {}\nProblem 2 {}", problem_1(&games), problem_2(&games)}
}