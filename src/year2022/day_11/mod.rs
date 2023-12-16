use std::fmt::Display;

use num::Integer;

#[derive(Clone, Copy)]
enum Operation {
    Add(i64),
    Mult(i64),
    Square,
}

impl Operation {
    fn apply(&self, value: i64) -> i64 {
        match self {
            Operation::Add(x) => value + x,
            Operation::Mult(x) => value * x,
            Operation::Square => value * value,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.contains("*") {
            let (_, op_value) = value.split_once(" * ").unwrap();
            let op_result = op_value.parse::<i64>();
            if op_result.is_ok() {
                Operation::Mult(op_result.unwrap())
            } else {
                Operation::Square
            }
        } else {
            let (_, op_value) = value.split_once(" + ").unwrap();
            let op = op_value.parse::<i64>().unwrap();
            Operation::Add(op)
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisible_by: i64,
    true_dest: usize,
    false_dest: usize,
}

fn parse_monkey(block: &str) -> Monkey {
    let mut items = vec![];
    let mut operation_opt = None;
    let mut divisible_by = -1;
    let mut true_dest_opt = None;
    let mut false_dest_opt = None;
    for line in block.lines() {
        if line.contains("Starting") {
            let (_, num_list) = line.split_once(": ").unwrap();
            items.extend(num_list.split(", ").map(|x| x.parse::<i64>().unwrap()));
        } else if line.contains("Operation") {
            operation_opt = Some(Operation::from(line));
        } else if line.contains("Test:") {
            let (_, num_part) = line.split_once("by ").unwrap();
            divisible_by = num_part.parse::<i64>().unwrap();
        } else if line.contains("true:") {
            let (_, num_part) = line.split_once("monkey ").unwrap();
            true_dest_opt = Some(num_part.parse::<usize>().unwrap());
        } else if line.contains("false:") {
            let (_, num_part) = line.split_once("monkey ").unwrap();
            false_dest_opt = Some(num_part.parse::<usize>().unwrap());
        }
    }
    Monkey {
        items,
        operation: operation_opt.unwrap(),
        divisible_by,
        true_dest: true_dest_opt.unwrap(),
        false_dest: false_dest_opt.unwrap(),
    }
} 

fn monkey_business<T: Integer + Copy + Display>(values: &Vec<T>) -> T {
    let mut top: (Option<T>, Option<T>) = (None, None);
    for i in values {
        if top.0.is_none() || i > &top.0.unwrap() {
            top.1 = top.0;
            top.0 = Some(*i);
        } else if top.1.is_none() || i > &top.1.unwrap() {
            top.1 = Some(*i);
        }
    }
    top.0.unwrap() * top.1.unwrap()
}

fn inspection_counter(monkeys: &mut Vec<Monkey>, rounds: i64, divisor: i64) -> usize {
    let mut inspection_count = vec![0; monkeys.len()];
    let big_modulo = monkeys.iter().fold(1, |acc, x| acc * x.divisible_by);
    for _ in 0..rounds {
        for m_key in 0..monkeys.len() {
            let monkey = monkeys[m_key].clone();
            for i in 0..monkey.items.len() {
                let item = monkey.items[i];
                let new_item = if divisor == 1 {
                    monkey.operation.apply(item)
                } else {
                    monkey.operation.apply(item) / divisor
                } % big_modulo;
                if new_item % monkey.divisible_by == 0 {
                    monkeys[monkey.true_dest].items.push(new_item);
                } else {
                    monkeys[monkey.false_dest].items.push(new_item);
                }
            }
            inspection_count[m_key] = inspection_count[m_key] + monkey.items.len();
            monkeys[m_key].items = vec![];
        }
    }
    monkey_business(&inspection_count)
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut monkeys: Vec<Monkey> = contents.split("\nMonkey").map(parse_monkey).collect();
    let mut monkeys2 = monkeys.clone();
    format!(
        "Problem 1: {}\nProblem 2: {}",
        inspection_counter(&mut monkeys, 20, 3),
        inspection_counter(&mut monkeys2, 10000, 1),
    )
}
