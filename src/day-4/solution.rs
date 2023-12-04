use std::collections::HashSet;

fn row_to_numbers(row: &str) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let trimmed = row.trim();
    let split_nums = trimmed.split(" ");
    for num in split_nums {
        if num == "" {
            continue;
        }
        result.push(num.parse::<i32>().unwrap());
    }
    result
}

fn solve_game(row: String) -> i32 {
    let mut split = row.split("|");
    let winning_nums_vec = row_to_numbers(split.next().unwrap());
    let players_nums = row_to_numbers(split.next().unwrap());
    let winning_nums: HashSet<&i32> = HashSet::from_iter(winning_nums_vec.iter());
    let mut count: i32 = 0;
    for num in players_nums {
        if winning_nums.contains(&num) {
            count += 1;
        }
    }
    count
}

fn solve_game_pow(row: String) -> i32 {
    let mut count: i32 = solve_game(row);
    count -= 1;
    if count == -1 {
        return 0;
    }
    let base: i32 = 2;
    base.pow(count.try_into().unwrap())
}

fn problem_1(contents: &str) -> i32 {
    let lines = contents.split("\n");
    let mut result_1 = 0;
    for line in lines {
        let mut split = line.split(":");
        let _ = split.next().unwrap();
        let game = split.next().unwrap();
        let game_result = solve_game_pow(game.to_string());
        result_1 += game_result;
    }
    result_1
}

fn problem_2(contents: &str) -> i32 {
    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    let card_count = lines.len();
    let mut count_vector = vec![1; card_count];
    for (i, line) in lines.iter().enumerate() {
        let mut split = line.split(":");
        let _ = split.next().unwrap();
        let game = split.next().unwrap();
        let score = solve_game(game.to_string());

        let current_count = count_vector[i];
        if score > 0 {

            for j in 1..score + 1 {
                let index: usize = i + usize::try_from(j).unwrap();
                count_vector[index] += current_count;
            }
        }
    }
    count_vector.iter().sum()
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let result_1 = problem_1(contents);
    let result_2 = problem_2(contents);
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}