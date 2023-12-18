struct Instruction {
    direction: u8,
    count: i64,
    colour: u32,
}

impl Instruction {
    fn from_colour(&self) -> Self {
        let direction_basis = self.colour % 16;
        let direction = match direction_basis {
            0 => b'R',
            1 => b'D',
            2 => b'L',
            3 => b'U',
            _ => panic!(""),
        };
        Instruction {
            direction,
            count: (self.colour / 16) as i64,
            colour: 0,
        }
    }
}

fn parse(contents: &str) -> Vec<Instruction> {
    contents.lines().map(
        |l| {
            let mut split = l.split(&[' ', '(', '#', ')']).filter(|s| s.len() > 0);
            Instruction {
                direction: split.next().unwrap().as_bytes()[0],
                count: split.next().unwrap().parse::<i64>().unwrap(),
                colour: u32::from_str_radix(split.next().unwrap(), 16).unwrap(),
            }
        }
    ).collect()
}

fn base_problem(instructions: &Vec<Instruction>) -> i64 {
    let (mut x, mut y, mut total) = (0, 0, 2);
    let mut edges: Vec<(i64, i64)> = instructions.iter().map(
        |inst| {
            match inst.direction {
                b'U' => y -= inst.count,
                b'D' => y += inst.count,
                b'L' => x -= inst.count,
                _ => x += inst.count,
            }
            total += inst.count;
            (x, y)
        }
    ).collect();
    edges.insert(0, (0, 0));
    edges.push(edges[0]);
    (edges.windows(2).map(|pair| pair[0].0 * pair[1].1 - pair[0].1 * pair[1].0).sum::<i64>() + total) / 2
}

fn problem_2(instructions: &Vec<Instruction>) -> i64 {
    let new_instructions = instructions.iter().map(|i| i.from_colour()).collect();
    base_problem(&new_instructions)
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let instructions = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", base_problem(&instructions), problem_2(&instructions))
}