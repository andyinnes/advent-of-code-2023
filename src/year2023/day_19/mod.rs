use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Clone, Copy, Debug)]
struct PartRange {
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
}

impl PartRange {
    fn product(&self) -> i128 {
        let x = (self.x.1 - self.x.0 + 1) as i128;
        let m = (self.m.1 - self.m.0 + 1) as i128;
        let a = (self.a.1 - self.a.0 + 1) as i128;
        let s = (self.s.1 - self.s.0 + 1) as i128;
        x * m * a * s
    }
    fn get_range(&self, field: &String) -> (i32, i32) {
        if field == "x" {
            self.x
        } else if field == "m" {
            self.m
        } else if field == "a" {
            self.a
        } else {
            self.s
        }
    }
    fn set_range(&self, field: &String, range: (i32, i32)) -> PartRange {
        if field == "x" {
            PartRange {
                x: range,
                ..self.clone()
            }
        } else if field == "m" {
            PartRange {
                m: range,
                ..self.clone()
            }
        } else if field == "a" {
            PartRange {
                a: range,
                ..self.clone()
            }
        } else {
            PartRange {
                s: range,
                ..self.clone()
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Outcome {
    Workflow(String),
    Accepted,
    Rejected,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "A" => Outcome::Accepted,
            "R" => Outcome::Rejected,
            x => Outcome::Workflow(x.to_string()),
        }
    }
}

struct Rule {
    field: Option<String>,
    gt: bool,
    comp_value: i32,
    outcome: Outcome,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<Outcome> {
        let value = if self.field.is_some() {
            let field = self.field.as_ref().unwrap();
            if field == "x" {
                part.x
            } else if field == "m" {
                part.m
            } else if field == "a" {
                part.a
            } else {
                part.s
            }
        } else {
            return Some(self.outcome.clone());
        };
        if self.gt {
            if value > self.comp_value {
                return Some(self.outcome.clone());
            }
        } else {
            if value < self.comp_value {
                return Some(self.outcome.clone());
            }
        }
        None
    }

    fn apply_range(&self, part: &PartRange) -> Vec<(PartRange, Option<Outcome>)> {
        let mut output = vec![];
        if self.field.is_none() {
            output.push((part.clone(), Some(self.outcome.clone())));
            return output;
        }
        let field = self.field.as_ref().unwrap();
        let current_range = part.get_range(self.field.as_ref().unwrap());
        if self.gt {
            if current_range.0 > self.comp_value {
                output.push((part.clone(), Some(self.outcome.clone())))
            } else if current_range.1 > self.comp_value {
                output.push((part.set_range(&field, (current_range.0, self.comp_value)), None));
                output.push((part.set_range(&field, (self.comp_value + 1, current_range.1)), Some(self.outcome.clone())))
            } else {
                output.push((part.clone(), None))
            }
        } else {
            if current_range.1 < self.comp_value {
                output.push((part.clone(), Some(self.outcome.clone())))
            } else if current_range.0 < self.comp_value {
                output.push((part.set_range(&field, (current_range.0, self.comp_value - 1)), Some(self.outcome.clone())));
                output.push((part.set_range(&field, (self.comp_value, current_range.1)), None))
            } else {
                output.push((part.clone(), None))
            }
        }
        output
    }
}

fn parse_workflows(workflows_file: &str) -> HashMap<String, Vec<Rule>> {
    let mut output = HashMap::new();
    for line in workflows_file.lines() {
        let (name, remainder) = line.split_once("{").unwrap();
        let mut current_workflow = vec![];
        let workflow = remainder.replace("}", "");
        for rule_str in workflow.split(",") {
            if rule_str.contains(":") {
                let (comp, outcome_str) = rule_str.split_once(":").unwrap();
                let outcome = Outcome::from(outcome_str);
                if comp.contains("<") {
                    let (field, comp_str) = comp.split_once("<").unwrap();
                    let comp_value = comp_str.parse::<i32>().unwrap();
                    current_workflow.push(
                        Rule {
                            field: Some(field.to_string()),
                            gt: false,
                            comp_value,
                            outcome,
                        }
                    )
                } else {
                    let (field, comp_str) = comp.split_once(">").unwrap();
                    let comp_value = comp_str.parse::<i32>().unwrap();
                    current_workflow.push(
                        Rule {
                            field: Some(field.to_string()),
                            gt: true,
                            comp_value,
                            outcome,
                        }
                    )
                }
            } else {
                let outcome = Outcome::from(rule_str);
                current_workflow.push(
                    Rule {
                        field: None,
                        gt: false,
                        comp_value: 0,
                        outcome,
                    }
                );
            }
        }
        output.insert(name.to_string(), current_workflow);
    }
    output
}

fn parse_parts(parts_str: &str) -> Vec<Part> {
    parts_str
        .lines()
        .map(
            |line|
            line
            .split(['{', '=', ',', 'x', 'm', 'a', 's', '}'])
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .map(|row| Part { x: row[0], m: row[1], a: row[2], s: row[3] })
        .collect()
}

fn parse(contents: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (rules, parts_str) = contents.split_once("\n\n").unwrap();
    let workflows = parse_workflows(rules);
    let parts = parse_parts(parts_str);
    (workflows, parts)
}

fn problem_1(workflows: &HashMap<String, Vec<Rule>>, parts: &Vec<Part>) -> i32 {
    let mut result = 0;
    for part in parts {
        let mut current_workflow = Some(&workflows[&"in".to_string()]);
        while current_workflow.is_some() {
            let mut outcome = None;
            for rule in current_workflow.unwrap() {
                outcome = rule.apply(part);
                if outcome.is_some() {
                    break;
                }
            }
            match outcome {
                Some(out) => {
                    match out {
                        Outcome::Accepted => {
                            result += part.x + part.m + part.a + part.s;
                            current_workflow = None;
                        },
                        Outcome::Rejected => {
                            current_workflow = None;
                        },
                        Outcome::Workflow(w) => {
                            current_workflow = Some(&workflows[&w]);
                        }
                    }
                }
                None => panic!(""),
            }
        }
    }
    result
}


fn push_range_to_queue(
    range: &PartRange, outcome: &Outcome, ranges_to_visit: &mut Vec<(PartRange, String)>, accepted: &mut Vec<PartRange>
) -> () {
    match outcome {
        Outcome::Accepted => {
            accepted.push(range.clone());
        }
        Outcome::Workflow(x) => {
            ranges_to_visit.push((range.clone(), x.clone()));
        },
        _ => (),
    };
}

fn problem_2(workflows: &HashMap<String, Vec<Rule>>) -> i128 {
    let mut ranges: Vec<(PartRange, String)> = vec![];
    let mut accepted_ranges: Vec<PartRange> = vec![];
    ranges.push((
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in".to_string(),
    ));
    let mut counter = 0;
    while counter < ranges.len() {
        let mut current_range = Some(ranges[counter].0);
        let current_workflow = &workflows[&ranges[counter].1];
        for rule in current_workflow {
            let new_ranges = rule.apply_range(&current_range.unwrap());
            current_range = None;
            for n_r in new_ranges {
                if n_r.1.is_none() {
                    current_range = Some(n_r.0);
                } else {
                    push_range_to_queue(&n_r.0, &n_r.1.unwrap(), &mut ranges, &mut accepted_ranges);
                }
            }
        }
        counter += 1;
    }
    let mut result: i128 = 0;
    for r in accepted_ranges {
        result += r.product();
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let (workflows, parts) = parse(contents);
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&workflows, &parts), problem_2(&workflows))
}