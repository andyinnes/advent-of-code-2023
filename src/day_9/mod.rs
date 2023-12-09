struct SequenceValues { previous: i32, next: i32 }

fn next_and_prev_in_sequence(sequence: &Vec<i32>) -> SequenceValues {
    let sequence_len = sequence.len();
    if sequence_len == 1 {
        return SequenceValues {previous: sequence[0], next: sequence[0] };
    }
    if sequence[1] - sequence[0] == sequence[sequence_len - 1] - sequence[sequence_len - 2] {
        // Could do with recursion, quicker to do this way (maybe (I checked, maybe slightly quicker but negligible))
        return SequenceValues { previous: 2 * sequence[0] - sequence[1], next: 2 * sequence[sequence_len - 1] - sequence[sequence_len - 2]};
    }
    if sequence.iter().all(|x| *x == 0) {
        return SequenceValues { previous: 0, next: 0 };
    }
    let mut diff_vector = vec![];
    for i in 1..sequence_len {
        diff_vector.push(sequence[i] - sequence[i - 1]);
    }
    let sequence_values = next_and_prev_in_sequence(&diff_vector);
    SequenceValues { previous: sequence[0] - sequence_values.previous, next: sequence[sequence_len - 1] + sequence_values.next }
}

fn parse(contents: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    for line in contents.lines() {
        let current_line = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        output.push(current_line);
    }
    output
}

fn problem_loop(readings: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut result = 0;
    let mut result_2 = 0;
    for reading in readings {
        let sequence_values = next_and_prev_in_sequence(reading);
        result += sequence_values.next;
        result_2 += sequence_values.previous;
    }
    (result, result_2)
}

pub fn solution() -> String {
    let readings = parse(include_str!("input.txt"));
    let (result_1, result_2) = problem_loop(&readings);
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}