fn parse(contents: &str) -> Vec<i32> {
    let mut all_values = vec![];
    let mut current = 0;
    for line in contents.lines() {
        if line == "" {
            all_values.push(current);
            current = 0;
            continue;
        }
        current += line.parse::<i32>().unwrap();
    }
    all_values
}

fn problem_2(calories: &Vec<i32>) -> i32 {
    let (mut max1, mut max2, mut max3) = (&0, &0, &0);
    for cal in calories {
        if cal > max1 {
            (max1, max2, max3) = (cal, max1, max2);
        } else if cal > max2 {
            (max2, max3) = (cal, max2);
        } else if cal > max3 {
            max3 = cal;
        }
    }
    max1 + max2 + max3
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let calories = parse(contents);
    format!{"Problem 1 {}\nProblem 2 {}", calories.iter().max().unwrap(), problem_2(&calories)}
}