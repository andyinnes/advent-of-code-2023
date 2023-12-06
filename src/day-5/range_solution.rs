#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Range { start: i64, end: i64 }

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut sorted = ranges.clone();
    sorted.sort();
    let mut output: Vec<Range> = vec![];
    for range in sorted {
        if range.start == range.end {
            continue;
        }
        let output_len = output.len();
        if output_len == 0 || output[output_len - 1].end <= range.start {
            output.push(range);
        } else {
            // Here the ranges overlap
            let prev_range = output.pop().unwrap();
            output.push(Range { start: prev_range.start, end: range.end });
        }
    }
    output
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
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
    fn map_range(&self, range: Range) -> Vec<Range>;
    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range>;
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

    fn map_range(&self, range: Range) -> Vec<Range> {
        // Assumes the ranges here are already sorted
        let mut prev_end = 0;
        let mut output = vec![];
        for map in self {
            // Skip if too low
            if map.end <= range.start {
                prev_end = map.end; // Not important but better to be correct
                continue;
            }
            if range.start >= map.start {
                // Add in the remaining range if range start is geq the map start
                output.push(Range { start: range.start + map.shift, end: map.end.min(range.end) + map.shift });
                prev_end = map.end;
                if map.end >= range.end {
                    // If this is reached, then this map is the final relevant map, so break
                    break;
                }
            } else {
                // Add in the range from the before this mapping started, up to the new map (or end of range) as unshifted
                let unshifted_range = Range { start: range.start.max(prev_end), end: map.start.min(range.end) };
                if unshifted_range.start < unshifted_range.end {
                    output.push(unshifted_range)
                }
                prev_end = map.end;
                if range.end > map.start {
                    output.push(Range { start: map.start + map.shift, end: map.end.min(range.end) + map.shift });
                } else {
                    // If this is reached, it means the current map is to high up to impact this range, so it and any after are not relevant
                    break;
                }
            }
        }
        // Special cases.
        if output.len() == 0 {
            // If not ranges have been mapped at all, then this means no maps where reached where the map was big enough, so just return the original range
            // equiv to prev_end <= range.start
            return vec![range];
        }
        if prev_end < range.end {
            output.push(Range { start: prev_end, end: range.end });
        }
        merge_ranges(&output)
    }

    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut full_ranges = vec![];
        for range in ranges {
            let mut current_mapped_ranges = self.map_range(range);
            full_ranges.append(&mut current_mapped_ranges);
        }
        merge_ranges(&full_ranges)
    }
}

struct SeedMapsTuple (Vec<SingleMap>, Vec<SingleMap>, Vec<SingleMap>, Vec<SingleMap>, Vec<SingleMap>, Vec<SingleMap>, Vec<SingleMap>);

impl SeedMapsTuple {
    fn map_value(&self, value: i64) -> i64 {
        let mut output: i64 = value;
        output = self.0.shift_value(output);
        output = self.1.shift_value(output);
        output = self.2.shift_value(output);
        output = self.3.shift_value(output);
        output = self.4.shift_value(output);
        output = self.5.shift_value(output);
        output = self.6.shift_value(output);
        output
    }

    fn map_range(&self, range: Range) -> Vec<Range> {
        let mut output: Vec<Range> = vec![range];
        output = self.0.map_ranges(output);
        output = self.1.map_ranges(output);
        output = self.2.map_ranges(output);
        output = self.3.map_ranges(output);
        output = self.4.map_ranges(output);
        output = self.5.map_ranges(output);
        output = self.6.map_ranges(output);
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
    new_map.sort();
    (new_map, end)
}

fn parse_seed_maps(line_iterable: &Vec<&str>) -> SeedMapsTuple {
    let (map1, end1) = create_mapping(line_iterable, 0);
    let (map2, end2) = create_mapping(line_iterable, end1);
    let (map3, end3) = create_mapping(line_iterable, end2);
    let (map4, end4) = create_mapping(line_iterable, end3);
    let (map5, end5) = create_mapping(line_iterable, end4);
    let (map6, end6) = create_mapping(line_iterable, end5);
    let (map7, _) = create_mapping(line_iterable, end6);
    SeedMapsTuple (map1, map2, map3, map4, map5, map6, map7)
}

fn problem_1(seeds: &Vec<i64>, seed_maps: &SeedMapsTuple) -> i64 {
    let mut result = i64::MAX;
    for seed in seeds {
        result = result.min(seed_maps.map_value(*seed));
    }
    result
}

fn problem_2(seeds: &Vec<i64>, seed_maps: &SeedMapsTuple) -> i64 {
    let mut result = i64::MAX;
    let max_lower_index: usize = seeds.len() - 1;
    for i in 0..max_lower_index {
        // Lazy approach with the indices
        let lower = 2 * i;
        let upper = lower + 1;
        if lower >= max_lower_index {
            break;
        }
        let output_ranges = seed_maps.map_range(Range {start: seeds[lower], end: seeds[lower] + seeds[upper]});
        result = result.min(output_ranges.iter().map(|x| x.start).min().unwrap());
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