use std::collections::HashMap;
use itertools::Itertools;

struct Instruction {
    count: i32,
    from: i32,
    to: i32,
}

impl Instruction {
    fn apply_instruction(&self, crane_state: &mut HashMap<i32, Vec<String>>, reverse: bool) -> () {
        let mut extend_vec = vec![];
        for _ in 0..self.count {
            extend_vec.push(crane_state.get_mut(&self.from).unwrap().pop().unwrap());
        }
        if reverse {
            extend_vec.reverse();
            crane_state.get_mut(&self.to).unwrap().extend(extend_vec)
        } else {
            crane_state.get_mut(&self.to).unwrap().extend(extend_vec)
        }
    }
}

fn parse_crane(lines: &Vec<&str>) -> HashMap<i32, Vec<String>> {
    let mut crane_state = HashMap::new();
    let crane_num_lines = lines[lines.len() - 1];
    let last_id = crane_num_lines.trim().split_whitespace().nth_back(0).unwrap();
    let last_id_value = last_id.parse::<i32>().unwrap();
    for i in 1..last_id_value + 1 {
        let pos = (-3 + 4 * i) as usize;
        let mut crane = vec![];
        for j in 0..lines.len() - 1 {
            let line = lines[j];
            let c = line.chars().nth(pos);
            if c != None && c.unwrap() != ' ' {
                crane.insert(0, c.unwrap().to_string());
            }
        }
        crane_state.insert(i, crane.clone());
    }
    crane_state
}

fn parse_instructions(lines: &Vec<&str>) -> Vec<Instruction> {
    let mut output = vec![];
    for line in lines {
        let mut first_split = line.split(" from ");
        let count = first_split.next().unwrap().replace("move ", "").parse::<i32>().unwrap();
        let mut second_split = first_split.next().unwrap().split(" to ");
        let from = second_split.next().unwrap().parse::<i32>().unwrap();
        let to = second_split.next().unwrap().parse::<i32>().unwrap();
        output.push(Instruction { count, from, to });
    }
    output
}

fn parse(contents: &str) -> (HashMap<i32, Vec<String>>, Vec<Instruction>) {
    let mut crane_lines = vec![];
    let mut instruction_lines = vec![];
    let mut in_crane = true;
    for line in contents.lines() {
        if line == "" {
            in_crane = false;
            continue;
        }
        if in_crane {
            crane_lines.push(line);
        } else {
            instruction_lines.push(line);
        }
    }
    let crane_state = parse_crane(&crane_lines);
    let instructions = parse_instructions(&instruction_lines);
    (crane_state, instructions)

}

fn apply_instructions(crane_state: &mut HashMap<i32, Vec<String>>, instructions: &Vec<Instruction>, reverse: bool) -> () {
    for instruction in instructions {
        instruction.apply_instruction(crane_state, reverse);
    }
}

fn output_string(crane_state: &HashMap<i32, Vec<String>>) -> String {
    let mut result = String::new();
    for (_k, v) in crane_state.iter().sorted() {
        result.push_str(&v[v.len() - 1]);
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (mut state_1, instructions) = parse(contents);
    let mut state_2 = state_1.clone();
    apply_instructions(&mut state_1, &instructions, false);
    apply_instructions(&mut state_2, &instructions, true);
    format!{"Problem 1 {}\nProblem 2 {}", output_string(&state_1), output_string(&state_2)}
}