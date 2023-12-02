fn convert_to_nums(val: String) -> String {
    let mut output: String = String::from("");
    for (i, c) in val.chars().enumerate() {
        if c.is_digit(10) {
            output.push_str(&c.to_string());
        }
        let substring = &val[i..];
        if substring.starts_with("zero") {output.push_str("0");}
        else if substring.starts_with("one") {output.push_str("1");}
        else if substring.starts_with("two") {output.push_str("2");}
        else if substring.starts_with("three") {output.push_str("3");}
        else if substring.starts_with("four") {output.push_str("4");}
        else if substring.starts_with("five") {output.push_str("5");}
        else if substring.starts_with("six") {output.push_str("6");}
        else if substring.starts_with("seven") {output.push_str("7");}
        else if substring.starts_with("eight") {output.push_str("8");}
        else if substring.starts_with("nine") {output.push_str("9");}
    }
    output
}

fn get_value(mut val: String, replace: bool) -> i32 {
    if replace {
        val = convert_to_nums(val);
    }
    let mut first: char = ' ';
    let mut last: char = ' ';
    for c in val.chars() {
        if c.is_digit(10) {
            if first == ' ' {
                first = c;
                last = c;
            } else {
                last = c;
            }
        }
    }
    let output: String = format!("{first}{last}");
    let out_int: i32 = output.parse::<i32>().unwrap();
    out_int
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut result_1 = 0;
    let mut result_2 = 0;
    for line in contents.lines() {
        result_1 += get_value(line.to_string(), false);
        result_2 += get_value(line.to_string(), true);
    }
    let prob_1 = format!("Problem 1: {result_1}");
    let prob_2 = format!("Problem 2: {result_2}");
    format!("{prob_1}\n{prob_2}")
}

fn main() {
    println!("{}", solution());
}
