use std::collections::HashSet;
use itertools::Itertools;

struct Backpack {
    left: Vec<char>,
    right: Vec<char>,
}

impl Backpack {
    fn to_set(&self) -> HashSet<char> {
        let mut base: HashSet<char> = HashSet::from_iter(self.left.clone());
        base.extend(self.right.clone());
        base
    }

    fn matching(&self) -> char {
        let l: HashSet<char> = HashSet::from_iter(self.left.clone());
        let r: HashSet<char> = HashSet::from_iter(self.right.clone());
        let diff: Vec<&char> = l.intersection(&r).collect();
        if diff.len() != 1 {
            panic!("Unexpected mismatching backpack length");
        }
        diff[0].clone()
    }

    fn matching_3(&self, second: &Self, third: &Self) -> char {
        let base: HashSet<char> = self.to_set().intersection(&second.to_set()).map(|x| x.clone()).collect();
        let t: HashSet<char> = third.to_set();
        let matches: Vec<&char> = t.intersection(&base).collect();
        if matches.len() != 1 {
            panic!("Unexpected number of matches between backpacks");
        }
        matches[0].clone()
    }
}

fn parse(contents: &str) -> Vec<Backpack> {
    let mut output = vec![];
    for line in contents.lines() {
        let mid = line.len() / 2;
        let all_chars: Vec<char> = line.chars().collect();
        let (l, r) = all_chars.split_at(mid);
        output.push(Backpack { left: l.to_vec(), right: r.to_vec() });
    }
    output
}

fn score_char(c: &char) -> i32 {
    let ord = *c as i32;
    if ord >= 97 {
        ord - 96
    } else {
        ord - 38
    }

}

fn problem_1(backpacks: &Vec<Backpack>) -> i32 {
    let mut result = 0;
    for backpack in backpacks {
        result += score_char(&backpack.matching());
    }
    result
}

fn problem_2(backpacks: &Vec<Backpack>) -> i32 {
    let mut result = 0;
    for backpack_group in &backpacks.iter().chunks(3) {
        let group: Vec<&Backpack> = backpack_group.collect();
        if group.len() != 3 {
            panic!("Not enough backpacks in the group");
        }
        let c = group[0].matching_3(group[1], group[2]);
        result += score_char(&c);
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let backpacks = parse(contents);
    format!{"Problem 1 {}\nProblem 2 {}", problem_1(&backpacks), problem_2(&backpacks)}
}