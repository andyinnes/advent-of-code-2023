use std::collections::HashMap;

const DIR_LIMIT: i32 = 100000;
const DISK_SPACE: i32 = 70000000;
const SPACE_REQUIRED: i32 = 30000000;

fn parse(contents: &str) -> HashMap<String, i32> {
    let mut base_dir_size = 0;
    let mut current_dirs: HashMap<String, i32> = HashMap::new();
    let mut finished_dirs: HashMap<String, i32> = HashMap::new();
    let mut current_path: Vec<String> = vec![];
    for line in contents.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let replaced = line.replace("$ cd ", "");
                if replaced == "/" {
                    for (dir, size) in current_dirs.iter() {
                        finished_dirs.insert(dir.clone(), *size);
                    }
                    current_dirs = HashMap::new();
                    current_path = vec![];
                } else if replaced == ".." {
                    let dir = current_path.pop().unwrap();
                    finished_dirs.insert(dir.clone(), current_dirs.remove(&dir).unwrap_or(0));
                } else {
                    let mut full_name = current_path.join("/");
                    full_name.push_str(replaced.as_str());
                    current_dirs.insert(full_name.clone(), 0);
                    current_path.push(full_name);
                }
            }
         } else {
            if !line.starts_with("dir") {
                let mut split = line.split(" ");
                let size = split.next().unwrap().parse::<i32>().unwrap();
                base_dir_size += size;
                for dir in &current_path {
                    if !current_dirs.contains_key(dir) {
                        println!("Dir {dir}\n{line}\n{:?}", current_dirs);
                    }
                    current_dirs.insert(dir.clone(), current_dirs[dir] + size);
                }
            }
        }
    }
    for (dir, size) in current_dirs.iter() {
        finished_dirs.insert(dir.clone(), *size);
    }
    finished_dirs.insert("/".to_string(), base_dir_size);
    finished_dirs
}

fn problem_1(dir_sizes: &HashMap<String, i32>) -> i32 {
    let mut result = 0;
    for size in dir_sizes.values() {
        if size <= &DIR_LIMIT {
            result += size;
        }
    }
    result
}

fn problem_2(dir_sizes: &HashMap<String, i32>) -> i32 {
    let minimum_dir_size = SPACE_REQUIRED - (DISK_SPACE - dir_sizes["/"]);
    if minimum_dir_size <= 0 {
        return 0;
    }
    let mut minimum_dir_value = dir_sizes["/"] + 1;
    for size in dir_sizes.values() {
        if size >= &minimum_dir_size {
            minimum_dir_value = minimum_dir_value.min(*size);
        }
    }
    minimum_dir_value
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let dir_sizes = parse(contents);
    format!{"Problem 1 {:?}\nProblem 2 {}", problem_1(&dir_sizes), problem_2(&dir_sizes)}
}