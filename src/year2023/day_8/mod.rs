use std::collections::HashMap;
use num::integer::lcm;

struct Route {
    left: String,
    right: String,
}

fn parse(content: &str) -> (Vec<char>, HashMap<String, Route>) {
    let mut lines = content.lines();
    let instructions = lines.next().unwrap().chars().collect();

    let mut route_map = HashMap::new();
    for line in lines {
        if line == "" {
            continue;
        }
        let route_key = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        route_map.insert(route_key, Route { left, right });
    }
    (instructions, route_map)
}

fn problem_1(instructions: &Vec<char>, route_map: &HashMap<String, Route>, start: String, end_on_z: bool) -> usize {
    let mut steps = 0;
    let mut position = start;
    loop {
        if position == "ZZZ" || (end_on_z && position[2..3] == *"Z") {
            break;
        }
        let direction: char = instructions[steps % instructions.len()];
        if direction == 'L' {
            position = route_map[&position].left.clone();
        } else {
            position = route_map[&position].right.clone();
        }
        steps += 1;
    }
    steps
}

fn problem_2(instructions: &Vec<char>, route_map: &HashMap<String, Route>) -> usize {
    let positions: Vec<String> = route_map.keys().filter(|x| x.ends_with("A")).map(|x| x.clone()).collect();
    let dist_to_z: Vec<usize> = positions.iter().map(|x| problem_1(instructions, route_map, x.clone(), true)).collect();
    println!("{:?}", dist_to_z);
    let mut output = dist_to_z[0];
    for i in 1..dist_to_z.len() {
        output = lcm(output, dist_to_z[i]);
    }
    output
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (instructions, route_map) = parse(contents);
    let result_1 = problem_1(&instructions, &route_map, String::from("AAA"), false);
    let result_2 = problem_2(&instructions, &route_map);
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}
