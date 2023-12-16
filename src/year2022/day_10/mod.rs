fn sprite_contains(sprite: &(i32, i32, i32), position: i32) -> bool {
    let pos2 = position % 40;
    sprite.0 == pos2 || sprite.1 == pos2 || sprite.2 == pos2
}

fn problem(contents: &str) -> (i32, String) {
    let mut cycle = 0;
    let mut signal_strength = 1;
    let mut signal_output = 0;
    let mut current_target = 20;
    let mut sprite = (0, 1, 2);
    let mut string_output = String::new();
    let target_increment = 40;
    for line in contents.lines() {
        if line.contains("noop") {
            if sprite_contains(&sprite, cycle) {
                string_output += "#";
            } else {
                string_output += ".";
            }
            cycle += 1;
            if cycle > current_target {
                signal_output += signal_strength * current_target;
                current_target += target_increment;
            }
        } else {
            for shift in 0..2 {
                if sprite_contains(&sprite, cycle + shift) {
                    string_output += "#";
                } else {
                    string_output += ".";
                }
            }
            let (_, op_amount) = line.split_once(' ').unwrap();
            let op_value = op_amount.parse::<i32>().unwrap();
            cycle += 2;
            if cycle >= current_target {
                signal_output += signal_strength * current_target;
                current_target += target_increment;
            }
            sprite = (sprite.0 + op_value, sprite.1 + op_value, sprite.2 + op_value);
            signal_strength += op_value;
        }
    }
    (signal_output, string_output)
}

fn problem_2_print(tv_screen: &String) -> String {
    let mut output = String::new();
    for (i, c) in tv_screen.chars().enumerate() {
        if i % 40 == 0 {
            output += "\n";
        }
        output.push(c);
    }
    output
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (problem_1, output) = problem(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1, problem_2_print(&output))
}
