use std::iter::zip;

fn parse(contents: &str) -> (Vec<u64>, Vec<u64>) {
    let mut times = vec![];
    let mut records = vec![];
    for line in contents.lines() {
        let mut new_line = String::new();
        let is_time = line.starts_with("Time:");
        if is_time {
            new_line = line.replace("Time:", "");
        } else {
            new_line = line.replace("Distance:", "");
        }
        new_line = new_line.trim().to_string();
        let current_vector: Vec<u64> = new_line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        if is_time {
            times.extend(current_vector);
        } else {
            records.extend(current_vector);
        }
    }
    (times, records)
}

fn concat_vector(data: &Vec<u64>) -> u64 {
    let mut data_string = String::new();
    for value in data {
        data_string = format!("{data_string}{value}");
    }
    data_string.parse::<u64>().unwrap()
}

fn distance(total_time: &u64, charge_time: &u64) -> u64 {
    if charge_time >= total_time {
        return 0;
    }
    (total_time - charge_time) * charge_time
}

fn solution_count(time: &u64, record: &u64) -> u64 {
    let max_check = time / 2;
    let mut min_a = 0;
    for a in 0..=max_check {
        let current_distance = distance(time, &a);
        if current_distance > *record {
            min_a = a;
            break;
        }
    }
    if min_a == 0 {
        return 0;
    }
    time - 2 * min_a + 1
}

fn problem_1(times: &Vec<u64>, records: &Vec<u64>) -> u64 {
    // This is symmetrical, so we find the minimum solution and can then find the max from there
    let mut result = 1;
    let zipped = zip(times, records);
    for (time, record) in zipped {
        result *= solution_count(&time, &record);
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (times, records) = parse(contents);
    let result_1 = problem_1(&times, &records);
    let single_time = concat_vector(&times);
    let single_distance = concat_vector(&records);
    let result_2 = solution_count(&single_time, &single_distance);
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}