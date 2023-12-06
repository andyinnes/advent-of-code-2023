struct SingleMap {
    start: i64,
    end: i64,
    shift: i64,
}

impl SingleMap {
    fn is_valid(&self, value: i64) -> bool {
        self.start <= value && value < self.end
    }
}

trait ShiftValue {
    fn shift_value(&self, value: i64) -> i64;
}

impl ShiftValue for Vec<SingleMap> {
    fn shift_value(&self, value: i64) -> i64 {
        for map in self {
            if map.is_valid(value) {
                return value + map.shift;
            }
        }
        value
    }
}

struct SeedMaps {
    seed_to_soil: Vec<SingleMap>,
    soil_to_fertilizer: Vec<SingleMap>,
    fertilizer_to_water: Vec<SingleMap>,
    water_to_light: Vec<SingleMap>,
    light_to_temperature: Vec<SingleMap>,
    temperature_to_humidity: Vec<SingleMap>,
    humidity_to_location: Vec<SingleMap>,
}

impl SeedMaps {
    fn map_value(&self, value: i64) -> i64 {
        let mut output: i64 = value;
        output = self.seed_to_soil.shift_value(output);
        output = self.soil_to_fertilizer.shift_value(output);
        output = self.fertilizer_to_water.shift_value(output);
        output = self.water_to_light.shift_value(output);
        output = self.light_to_temperature.shift_value(output);
        output = self.temperature_to_humidity.shift_value(output);
        output = self.humidity_to_location.shift_value(output);
        output
    }
}

fn get_seeds(line: String) -> Vec<i64> {
    let seeds_only = line.replace("seeds: ", "");
    let mut output = vec![];
    for seed_str in seeds_only.split_whitespace() {
        let seed = seed_str.parse::<i64>().unwrap();
        output.push(seed);
    }
    output
}

fn add_map_from_str(base_map: &mut Vec<SingleMap>, line: &str) -> () {
    let values: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
    if values.len() != 3 {
        panic!("Wrong number of values in seed map!!!");
    }
    let mut new_maps = vec![];
    let start = values[1];
    let end = start + values[2];
    let shift = values[0] - start;
    let map = SingleMap { start, end, shift };
    new_maps.push(map);
    base_map.extend(new_maps);
}

fn create_mapping(line_iterable: &Vec<&str>, start: usize) -> (Vec<SingleMap>, usize) {
    let mut new_map = vec![];
    let mut end = 0;
    for i in start.. {
        if i >= line_iterable.len() {
            end = i + 1;
            break;
        }
        let line = line_iterable[i];
        if line == "" {
            end = i + 1;
            break;
        }
        if line.ends_with("map:") {
            continue;
        }
        add_map_from_str(&mut new_map, line);
    }
    (new_map, end)
}

fn parse_seed_maps(line_iterable: &Vec<&str>) -> SeedMaps {
    let (map1, end1) = create_mapping(line_iterable, 0);
    let (map2, end2) = create_mapping(line_iterable, end1);
    let (map3, end3) = create_mapping(line_iterable, end2);
    let (map4, end4) = create_mapping(line_iterable, end3);
    let (map5, end5) = create_mapping(line_iterable, end4);
    let (map6, end6) = create_mapping(line_iterable, end5);
    let (map7, _) = create_mapping(line_iterable, end6);
    SeedMaps {
        seed_to_soil: map1,
        soil_to_fertilizer: map2,
        fertilizer_to_water: map3,
        water_to_light: map4,
        light_to_temperature: map5,
        temperature_to_humidity: map6,
        humidity_to_location: map7,
    }
}

fn problem_1(seeds: &Vec<i64>, seed_maps: &SeedMaps) -> i64 {
    let mut result = i64::MAX;
    for seed in seeds {
        result = result.min(seed_maps.map_value(*seed));
    }
    result
}

fn problem_2(seeds: &Vec<i64>, seed_maps: &SeedMaps) -> i64 {
    let mut result = i64::MAX;
    let max_lower_index = seeds.len() - 1;
    for i in 0..max_lower_index {
        println!("Started problem 2 block {i}");
        // Lazy approach with the indices
        let lower = 2 * i;
        let upper = lower + 1;
        if lower >= max_lower_index {
            break;
        }
        let start = seeds[lower];
        let end = start + seeds[upper];
        for seed_value in start..end {
            result = result.min(seed_maps.map_value(seed_value));
        }
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut lines_iter = contents.lines();
    let seeds = get_seeds(lines_iter.next().unwrap().to_string());
    lines_iter.next();
    let remaining_lines = lines_iter.collect();
    let seed_maps = parse_seed_maps(&remaining_lines);
    let result_1 = problem_1(&seeds, &seed_maps);
    let result_2 = problem_2(&seeds, &seed_maps);
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}