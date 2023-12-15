use std::collections::HashMap;

#[derive(Clone)]
struct Lens {
    name: String,
    focal_length: i32,
}

fn hash(word: &str) -> i32 {
    let mut output = 0;
    for c in word.chars().map(|c: char| c as i32) {
        output = (output + c) * 17 % 256;
    }
    output
}

fn base_map() -> HashMap<i32, Vec<Lens>> {
    let mut output = HashMap::new();
    for i in 0..256 {
        output.insert(i, vec![]);
    }
    output
}

fn insert_or_replace(lens: &Lens, bucket: &mut Vec<Lens>) -> () {
    let pos = bucket.iter().position(|x| x.name == lens.name);
    if pos == None {
        bucket.push(lens.clone());
    } else {
        bucket.get_mut(pos.unwrap()).unwrap().focal_length = lens.focal_length;
    }
}

fn apply_order(order: &str, map: &mut HashMap<i32, Vec<Lens>>) -> () {
    if order.contains("=") {
        let (lens, focal_length_str) = order.split_once("=").unwrap();
        let hashed = hash(lens);
        let focal_length = focal_length_str.parse::<i32>().unwrap();
        let lens = Lens { name: lens.to_string(),  focal_length };
        map.entry(hashed).and_modify(|x| insert_or_replace(&lens, x));
    } else {
        let (lens, _) = order.split_once("-").unwrap();
        let lens_string = lens.to_string();
        let hashed = hash(lens);
        map.entry(hashed).and_modify(|x| x.retain(|y| y.name != lens_string));
    }
}

fn problem_2(data: &str) -> i32 {
    let mut map = base_map();
    let mut output = 0;
    for order in data.split(",") {
        apply_order(order, &mut map);
    }
    for (key, bucket) in map.iter() {
        for (i, lens) in bucket.iter().enumerate() {
            output += (key + 1) * (i as i32 + 1) * lens.focal_length;
        }
    }
    output
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let problem_1: i32 = contents.split(",").map(hash).sum();
    format!("Problem 1: {}\nProblem 2: {}", problem_1, problem_2(contents))
}