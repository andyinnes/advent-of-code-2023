use std::collections::{HashMap, HashSet};

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum CoordType {
    Pipe,
    Dash,
    L,
    J,
    F,
    Seven,
    Start,
    Empty,
    Enclosed,
}

impl CoordType {
    fn allow_up(&self) -> bool {
        [CoordType::Pipe, CoordType::L, CoordType::J, CoordType::Start].contains(self)
    }
    fn allow_down(&self) -> bool {
        [CoordType::Pipe, CoordType::F, CoordType::Seven, CoordType::Start].contains(self)
    }
    fn allow_left(&self) -> bool {
        [CoordType::Dash, CoordType::J, CoordType::Seven, CoordType::Start].contains(self)
    }
    fn allow_right(&self) -> bool {
        [CoordType::Dash, CoordType::F, CoordType::L, CoordType::Start].contains(self)
    }
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct CoordBase {
    x: i32,
    y: i32,
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    coord_type: CoordType,
}

impl Coord {
    fn path_neighbours(&self) -> Vec<CoordBase> {
        let mut output = vec![];
        // Check if moving up is valid
        if self.coord_type.allow_up() {
            if self.y > 0 {
                output.push(CoordBase { x: self.x, y: self.y - 1});
            }
        }
        if self.coord_type.allow_down() {
            output.push(CoordBase { x: self.x, y: self.y + 1 });
        }
        if self.coord_type.allow_left() {
            if self.x > 0 {
                output.push(CoordBase { x: self.x - 1, y: self.y });
            }
        }
        if self.coord_type.allow_right() {
            output.push( CoordBase { x: self.x + 1, y: self.y });
        }
        output
    }

    fn neighbours(&self, limits: &CoordBase) -> Vec<CoordBase> {
        let mut output = vec![];
        if self.x > 0 {
            output.push(CoordBase { x: self.x - 1, y: self.y });
        }
        if self.y > 0 {
            output.push(CoordBase { x: self.x, y: self.y - 1 });
        }
        if self.x < limits.x {
            output.push(CoordBase { x: self.x + 1, y: self.y });
        }
        if self.y < limits.y {
            output.push(CoordBase { x: self.x, y: self.y + 1 });
        }
        output
    }

    fn allow_from(&self, from: &CoordBase) -> bool {
        self.path_neighbours().contains(from)
    }

    fn to_base(&self) -> CoordBase {
        CoordBase { x: self.x, y: self.y }
    }

    fn redefine_start(&self, next: &CoordBase, last: &CoordBase) -> Coord {
        if self.coord_type != CoordType::Start {
            panic!("");
        }
        let mut possible_types: HashSet<CoordType> = HashSet::new();
        if next.x != self.x {
            if next.x < self.x {
                possible_types.extend([CoordType::Dash, CoordType::J, CoordType::Seven]);
            } else {
                possible_types.extend([CoordType::Dash, CoordType::F, CoordType::L]);
            }
        } else {
            if next.y < self.y {
                possible_types.extend([CoordType::Pipe, CoordType::L, CoordType::J]);
            } else {
                possible_types.extend([CoordType::Pipe, CoordType::F, CoordType::Seven]);
            }
        }
        let type_set: HashSet<CoordType> = if last.x != self.x {
            if last.x < self.x {
                HashSet::from_iter([CoordType::Dash, CoordType::J, CoordType::Seven])
            } else {
                HashSet::from_iter([CoordType::Dash, CoordType::F, CoordType::L])
            }
        } else {
            if last.y < self.y {
                HashSet::from_iter([CoordType::Pipe, CoordType::L, CoordType::J])
            } else {
                HashSet::from_iter([CoordType::Pipe, CoordType::F, CoordType::Seven])
            }
        };
        let intersection: Vec<&CoordType> = possible_types.intersection(&type_set).collect();
        if intersection.len() != 1{
            panic!("Unexpected possible options for S type");
        }
        Coord { coord_type: *intersection[0], ..*self }
    }
}

fn parse(contents: &str) -> (HashMap<CoordBase, Coord>, Coord, CoordBase) {
    let mut data: HashMap<CoordBase, Coord> = HashMap::new();
    let mut start_coord = None;
    let mut limit_coord = CoordBase { x: 0, y: 0 };
    for (row_index, line) in contents.lines().enumerate(){
        let y = row_index as i32;
        limit_coord.y = y;
        for (i, step) in line.chars().enumerate() {
            let x = i as i32;
            limit_coord.x = x;
            let key = CoordBase { x, y };
            let base_value = Coord { x, y, coord_type: CoordType::Empty };
            if step == '|' {
                data.insert(key, Coord { coord_type: CoordType::Pipe, ..base_value });
            } else if step == '-' {
                data.insert(key, Coord { coord_type: CoordType::Dash, ..base_value });
            } else if step == 'L' {
                data.insert(key, Coord { coord_type: CoordType::L, ..base_value });
            } else if step == 'J' {
                data.insert(key, Coord { coord_type: CoordType::J, ..base_value });
            } else if step == '7' {
                data.insert(key, Coord { coord_type: CoordType::Seven, ..base_value });
            } else if step == 'F' {
                data.insert(key, Coord { coord_type: CoordType::F, ..base_value });
            } else if step == 'S' {
                let value = Coord { coord_type: CoordType::Start, ..base_value };
                data.insert(key, value);
                start_coord = Some(value);
            } else {
                data.insert(key, Coord { coord_type: CoordType::Empty, ..base_value });
            }
        }
    }
    (data, start_coord.unwrap(), limit_coord)
}

fn dfs_impl(graph: &HashMap<CoordBase, Coord>, position: &CoordBase, visited: &mut HashSet<CoordBase>, path: &mut Vec<CoordBase>) -> Option<Vec<CoordBase>> {
    if path.len() > 2 && path[0] == *position {
        return Some(path.clone());
    }
    if visited.contains(position) {
        return None;
    }
    visited.insert(*position);
    path.push(position.clone());
    let full_coord = graph[position];
    for new_node in full_coord.path_neighbours() {
        if graph.contains_key(&new_node) && graph[&new_node].allow_from(position) {
            let out = dfs_impl(graph, &new_node, visited, path);
            if out != None {
                return out;
            }
        }
    }
    path.pop();
    None
}

fn find_loop(graph: &HashMap<CoordBase, Coord>, start: &CoordBase) -> Vec<CoordBase> {
    let mut visited = HashSet::new();
    let mut current_path: Vec<CoordBase> = vec![];
    let path: Vec<CoordBase> = dfs_impl(graph, start, &mut visited, &mut current_path).unwrap();
    path
}

fn try_replace(graph: &mut HashMap<CoordBase, Coord>, path: &Vec<CoordBase>, coord: Coord) -> () {
    let key = coord.to_base();
    if graph.contains_key(&key) && !path.contains(&key) {
        graph.insert(key, coord);
    }
}

fn do_replacements(graph: &mut HashMap<CoordBase, Coord>, path: &Vec<CoordBase>, current: &Coord, above: bool) -> () {
    if current.coord_type == CoordType::Dash {
        let y = if above {
            current.y - 1
        } else {
            current.y + 1
        };
        for x in current.x - 1..=current.x + 1 {
            try_replace(graph, path, Coord { x, y, coord_type: CoordType::Enclosed });
        }
    } else if current.coord_type == CoordType::Dash {
        let x = if above {
            current.x - 1
        } else {
            current.x + 1
        };
        for y in current.y - 1..=current.y + 1 {
            try_replace(graph, path, Coord { x, y, coord_type: CoordType::Enclosed });
        }
    } else if current.coord_type == CoordType::L {
        if above {
            try_replace(graph, path, Coord {x: current.x + 1, y: current.y - 1, coord_type: CoordType::Enclosed });
        } else {
            let x = current.x - 1;
            for y in current.y - 1..=current.y + 1 {
                try_replace(graph, path, Coord { x, y, coord_type: CoordType::Enclosed });
            }
            try_replace(graph, path, Coord { x: current.x, y: current.y + 1, coord_type: CoordType::Enclosed });
            try_replace(graph, path, Coord { x: current.x + 1, y: current.y + 1, coord_type: CoordType::Enclosed });
        }
    } else if current.coord_type == CoordType::J {
        if above {
            try_replace(graph, path, Coord {x: current.x - 1, y: current.y - 1, coord_type: CoordType::Enclosed });
        } else {
            let x = current.x + 1;
            for y in current.y - 1..=current.y + 1 {
                try_replace(graph, path, Coord { x, y, coord_type: CoordType::Enclosed });
            }
            try_replace(graph, path, Coord { x: current.x - 1, y: current.y + 1, coord_type: CoordType::Enclosed });
            try_replace(graph, path, Coord { x: current.x, y: current.y + 1, coord_type: CoordType::Enclosed });
        }
    } else if current.coord_type == CoordType::Seven {
        if above {
            let x = current.x + 1;
            for y in current.y - 1..=current.y + 1 {
                try_replace(graph, path, Coord { x, y, coord_type: CoordType::Enclosed });
            }
            try_replace(graph, path, Coord { x: current.x - 1, y: current.y - 1, coord_type: CoordType::Enclosed });
            try_replace(graph, path, Coord { x: current.x, y: current.y - 1, coord_type: CoordType::Enclosed });
        } else {
            try_replace(graph, path, Coord {x: current.x - 1, y: current.y + 1, coord_type: CoordType::Enclosed });
        }
    } else if current.coord_type == CoordType::F {
        if above {
            let x = current.x - 1;
            for y in current.y - 1..=current.y + 1 {
                try_replace(graph, path, Coord { x, y, coord_type: CoordType::Enclosed });
            }
            try_replace(graph, path, Coord { x: current.x + 1, y: current.y - 1, coord_type: CoordType::Enclosed });
            try_replace(graph, path, Coord { x: current.x, y: current.y - 1, coord_type: CoordType::Enclosed });
        } else {
            try_replace(graph, path, Coord {x: current.x + 1, y: current.y + 1, coord_type: CoordType::Enclosed });
        }
    }
}

// fn print_graph(graph: &HashMap<CoordBase, Coord>) -> () {
//     let (max_x, max_y) = graph.keys().map(|k| (k.x, k.y)).max().unwrap();
//     for y in 0..=max_y {
//         for x in 0..=max_x {
//             let c = CoordBase {x, y};
//             print!("{}", graph[&c].coord_type.print());
//         }
//         print!("\n");
//     }
//     println!("Done!");
// }

fn problem_2(graph: &mut HashMap<CoordBase, Coord>, path: &Vec<CoordBase>, limits: &CoordBase) -> usize {
    // Replace anything not on the path with Empty. Also track the first path element found
    let mut first_element_opt = None;
    for y in 0..limits.y {
        for x in 0..limits.x {
            let key = CoordBase { x, y };
            try_replace(graph, path, Coord { x, y, coord_type: CoordType::Empty });
            if first_element_opt == None && graph[&key].coord_type != CoordType::Empty {
                first_element_opt = Some(key);
            }
        }
    }
    // Redefine the start node to be an actual coordinate
    let new_start = graph[&path[0]].redefine_start(&path[1], &path[path.len() - 1]);
    graph.insert(new_start.to_base(), new_start);
    // Get the first element of the path
    let first_element = first_element_opt.unwrap();
    // Roughly above is for where you would go if you next went into a dash. Let's say above is left of pipe
    let mut above = false;
    let mut prev_type: CoordType = CoordType::Pipe;

    let flip_map = vec![
        (CoordType::Pipe, CoordType::L),
        (CoordType::Pipe, CoordType::Seven),
        (CoordType::J, CoordType::Seven),
        (CoordType::L, CoordType::F),
        (CoordType::L, CoordType::Pipe),
        (CoordType::Seven, CoordType::Pipe),
        (CoordType::Seven, CoordType::J),
        (CoordType::F, CoordType::L),
    ];

    let start_index = path.iter().position(|x| x == &first_element).unwrap();
    let indices: Vec<usize> = (start_index..path.len()).chain(0..start_index).collect();
    for i in indices {
        let current_coord = path[i];
        let current = graph[&current_coord];
        let current_type = graph[&current_coord].coord_type;
        if flip_map.contains(&(prev_type, current_type)) {
            above = !above;
        }
        do_replacements(graph, path, &current, above);
        prev_type = current_type;
    }
    // Fill in the enclosed parts
    let mut enclosed = vec![];
    for (k, v) in graph.clone().iter() {
        if v.coord_type == CoordType::Enclosed {
            enclosed.push(k.clone());
        }
    }
    let mut pos = 0;
    let mut visited = HashSet::new();
    while pos < enclosed.len() {
        let coord = enclosed[pos];
        visited.insert(coord);
        let neighbours = graph[&coord].neighbours(limits);
        for n in neighbours {
            if !enclosed.contains(&n) {
                try_replace(graph, path, Coord { x: n.x, y: n.y, coord_type: CoordType::Enclosed });
                let new_type = graph[&n].coord_type;
                if new_type == CoordType::Enclosed {
                    enclosed.push(n);
                }
            }
        }
        pos += 1;
    }
    return enclosed.len()
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (mut graph, start, limits) = parse(contents);
    let path = find_loop(&graph, &start.to_base());
    let p2 = problem_2(&mut graph, &path, &limits);
    format!("Problem 1: {}\nProblem 2: {:?}", path.len() / 2, p2)
}