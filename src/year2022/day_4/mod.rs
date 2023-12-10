#[derive(Clone, Copy, Debug)]
struct PartRange {
    start: i32,
    end: i32,
}

impl PartRange {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start || self.start <= other.end && self.end >= other.start
    }
}

fn part_range_from_str(rng: &str) -> PartRange {
    let nums: Vec<i32> = rng.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    if nums.len() != 2 {
        panic!("Wrong number of elements in part range");
    }
    PartRange { start: nums[0], end: nums[1] }
}

fn parse(contents: &str) -> Vec<(PartRange, PartRange)> {
    let mut output = vec![];
    for line in contents.lines() {
        let part_ranges: Vec<PartRange> = line.split(",").map(|x| part_range_from_str(x)).collect();
        if part_ranges.len() != 2 {
            panic!("Wrong number of part ranges in line");
        }
        output.push((part_ranges[0], part_ranges[1]));
    }
    output
}

fn fully_contained_count(pairs: &Vec<(PartRange, PartRange)>) -> i32 {
    let mut count = 0;
    for pair in pairs {
        if pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0) {
            count += 1;
        }
    }
    count
}

fn overlap_count(pairs: &Vec<(PartRange, PartRange)>) -> i32 {
    let mut count = 0;
    for pair in pairs {
        if pair.0.overlaps(&pair.1) {
            count += 1;
        }
    }
    count
}

pub fn solution() -> String {
    let pairs = parse(include_str!("input.txt"));
    format!("Problem 1: {}\nProblem 2: {}", fully_contained_count(&pairs), overlap_count(&pairs))
}