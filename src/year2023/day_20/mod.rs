use num::integer::lcm;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    maps_to: Vec<String>,
    high: bool,
    input_fields: HashMap<String, bool>,
}

impl Module {
    fn send_signal(&mut self, origin: &String, high: bool) -> (Vec<String>, bool) {
        match self.module_type {
            ModuleType::Broadcast => {
                (self.maps_to.clone(), high)
            },
            ModuleType::Conjunction => {
                self.input_fields.insert(origin.clone(), high);
                if self.input_fields.values().all(|x| *x) {
                    (self.maps_to.clone(), false)
                } else {
                    (self.maps_to.clone(), true)                    
                }
            },
            ModuleType::FlipFlop => {
                if high {
                    (vec![], high)
                } else {
                    self.high = !self.high;
                    (self.maps_to.clone(), self.high)
                }
            }
        }
    }
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let (module_name, mappings) = value.split_once(" -> ").unwrap();
        let (name, module_type) = if module_name.starts_with("%") {
            (module_name.replace("%", ""), ModuleType::FlipFlop)
        } else if module_name.starts_with("&") {
            (module_name.replace("&", ""), ModuleType::Conjunction)
        } else {
            (module_name.to_string(), ModuleType::Broadcast)
        };
        let maps_to: Vec<String> = mappings.split(", ").map(|s| s.to_string()).collect();
        Module { name, module_type, maps_to, high: false, input_fields: HashMap::new() }
    }
}

fn parse(contents: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    contents.lines().map(Module::from).for_each(|m| {modules.insert(m.name.clone(), m);});
    let module_names: Vec<String> = modules.values().map(|item| item.name.clone()).collect();
    for module_name in module_names {
        for map_to_name in &modules[&module_name].maps_to.clone() {
            if modules.contains_key(map_to_name) {
                modules.get_mut(map_to_name).unwrap().input_fields.insert(module_name.clone(), false);
            }
        }
    }
    modules
}

fn problem(modules: &mut HashMap<String, Module>, required_field: &String, seen: &mut HashSet<String>) -> (i64, i64) {
    let mut module_steps = vec![];
    module_steps.push(("broadcaster".to_string(), false, "button".to_string()));
    let mut counter = 0;
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    while counter < module_steps.len() {
        let (module, high, origin) = &module_steps[counter];
        let md: String = module.clone();
        if *high {
            high_pulse_count += 1;
            if module == required_field {
                seen.insert(origin.clone());
            }
        } else {
            low_pulse_count += 1;
        }
        if modules.contains_key(module) {
            let (next_mods, next_high) = modules.get_mut(module).unwrap().send_signal(origin, *high);
            for next_mod in next_mods {
                module_steps.push((next_mod, next_high, md.clone()));
            }
        }
        counter += 1;
    }
    (low_pulse_count, high_pulse_count)
}

fn problem_1(modules: &mut HashMap<String, Module>) -> i64 {
    let mut lp_count = 0;
    let mut hp_count = 0;
    let dummy_string = String::new();
    let mut dummy_hashset = HashSet::new();
    for _ in 0..1000 {
        let (lp, hp) = problem(modules, &dummy_string, &mut dummy_hashset);
        lp_count += lp;
        hp_count += hp;
    }
    lp_count * hp_count
}

fn problem_2(modules: &mut HashMap<String, Module>) -> i64 {
    // Theory, there are a series of conjunctions that lead to a single conjunction feeding rx.
    // This should mean that the single low output is triggered when all of them return high.
    // Guessing, because day 8 benefited from guessing, that this will happen in cycles for each of the feeds
    // and the rx result will occur at the lcm.
    let expected_output = vec!["rx".to_string(); 1];
    let folded: Vec<String> = modules.iter().filter(|(_, x)| x.maps_to == expected_output).map(|(y, _)| y.clone()).collect();
    if folded.len() != 1 {
        panic!("Should have only 1 element mapping to rx");
    }
    let feed = folded[0].clone();
    let feed_feeds = modules.values().filter(|x| x.maps_to.contains(&feed)).count();
    println!("Feed: {feed}\nFeeds {feed_feeds}");
    let mut seen = HashSet::new();
    let mut cycle_lengths = HashMap::new();
    let mut counter = 1;
    while cycle_lengths.len() < feed_feeds {
        let (_, _) = problem(modules, &feed, &mut seen);
        for k in &seen {
            if !cycle_lengths.contains_key(k) {
                cycle_lengths.insert(k.clone(), counter);
            }
        }
        counter += 1;
    }
    let mut output = 1;
    for cycle in cycle_lengths.values() {
        output = lcm(output, *cycle);
    }
    output
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut modules = parse(contents);
    let mut modules_2 = modules.clone();
    format!("Problem 1: {}\nProblem 2: {}", problem_1(&mut modules), problem_2(&mut modules_2))
}