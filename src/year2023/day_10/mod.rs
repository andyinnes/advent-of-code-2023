use std::collections::{HashMap, HashSet};

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn above(&self) -> Self {
        Coord { x: self.x, y: self.y - 1 }
    }
    fn below(&self) -> Self {
        Coord { x: self.x, y: self.y + 1 }
    }
    fn left(&self) -> Self {
        Coord { x: self.x - 1, y: self.y }
    }
    fn right(&self) -> Self {
        Coord { x: self.x + 1, y: self.y }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Links {
    up: Option<Coord>,
    down: Option<Coord>,
    left: Option<Coord>,
    right: Option<Coord>,
}

fn parse(contents: &str) -> (HashMap<Coord, Links>, Coord, Coord) {
    let mut data: HashMap<Coord, Links> = HashMap::new();
    let mut start_coord = None;
    let mut limit_coord = Coord { x: 0, y: 0 };
    for (row_index, line) in contents.lines().enumerate(){
        let y = row_index as i32;
        limit_coord.y = y;
        for (i, step) in line.chars().enumerate() {
            let x = i as i32;
            limit_coord.x = x;
            let key = Coord { x, y };
            let all_links = Links {
                up: Some(Coord { x, y: y - 1 }),
                down: Some(Coord { x, y: y + 1 }),
                left: Some(Coord { x: x - 1, y }),
                right: Some(Coord { x: x + 1, y }),
            };
            if step == '|' {
                data.insert(key, Links { left: None, right: None, ..all_links});
            } else if step == '-' {
                data.insert(key, Links { up: None, down: None, ..all_links });
            } else if step == 'L' {
                data.insert(key, Links { left: None, down: None, ..all_links });
            } else if step == 'J' {
                data.insert(key, Links { right: None, down: None, ..all_links });
            } else if step == '7' {
                data.insert(key, Links { up: None, right: None, ..all_links });
            } else if step == 'F' {
                data.insert(key, Links { up: None, left: None, ..all_links });
            } else if step == 'S' {
                data.insert(key, all_links);
                start_coord = Some(key);
            }
        }
    }
    (data, start_coord.unwrap(), limit_coord)
}

fn dfs_impl(graph: &HashMap<Coord, Links>, position: &Coord, visited: &mut HashSet<Coord>, path: &mut Vec<Coord>) -> Option<Vec<Coord>> {
    if path.len() > 2 && path[0] == *position {
        return Some(path.clone());
    }
    if visited.contains(position) {
        return None;
    }
    visited.insert(*position);
    path.push(position.clone());
    let links = graph[position];
    if links.up != None && graph.contains_key(&links.up.unwrap()) {
        let new_node = links.up.unwrap();
        if graph.contains_key(&new_node) && graph[&new_node].down != None {
            let out = dfs_impl(graph, &new_node, visited, path);
            if out != None {
                return out;
            }
        }
    }
    if links.left != None && graph.contains_key(&links.left.unwrap()) {
        let new_node = links.left.unwrap();
        if graph.contains_key(&new_node) && graph[&new_node].right != None {
            let out = dfs_impl(graph, &new_node, visited, path);
            if out != None {
                return out;
            }
        }
    }
    if links.down != None && graph.contains_key(&links.down.unwrap()) {
        let new_node = links.down.unwrap();
        if graph.contains_key(&new_node) && graph[&new_node].up != None {
            let out = dfs_impl(graph, &new_node, visited, path);
            if out != None {
                return out;
            }
        }
    }
    if links.right != None && graph.contains_key(&links.right.unwrap()) {
        let new_node = links.right.unwrap();
        if graph.contains_key(&new_node) && graph[&new_node].left != None {
            let out = dfs_impl(graph, &new_node, visited, path);
            if out != None {
                return out;
            }
        }
    }
    path.pop();
    None
}

fn find_loop(graph: &HashMap<Coord, Links>, start: &Coord) -> Vec<Coord> {
    let mut visited = HashSet::new();
    let mut current_path: Vec<Coord> = vec![];
    let path: Vec<Coord> = dfs_impl(graph, start, &mut visited, &mut current_path).unwrap();
    path
}

// fn find_enclosed(graph: &HashMap<Coord, Links>, path: &Vec<Coord>, limits: Coord) -> i32 {
//     // Enclosed <=> Is connected to outside edge. So find everythin connected to the outside and find
//     // the complement of this.
//     let mut visited: HashSet<Coord> = HashSet::new();
//     let path_set: HashSet<&Coord> = HashSet::from_iter(path);
//     let mut to_visit: Vec<Coord> = vec![];
//     for x in 0..=limits.x {
//         to_visit.push(Coord { x, y: 0 });
//         to_visit.push(Coord { x, y: limits.y });
//     }
//     for y in 1..limits.x {
//         to_visit.push
//     }
//     0
// }

pub fn solution() -> String {
    let (graph, start, limits) = parse(include_str!("input.txt"));
    let path = find_loop(&graph, &start);
    format!("Problem 1: {}\nProblem 2: {}", path.len() / 2, 1)
}