use std::cmp::{max, min};

use crate::get_input_for_day;

const SEED_PREFIX: &str = "SEED";
const SOIL_PREFIX: &str = "SOIL";
const FERTILIZER_PREFIX: &str = "FERTILIZER";
const WATER_PREFIX: &str = "WATER";
const LIGHT_PREFIX: &str = "LIGHT";
const TEMP_PREFIX: &str = "TEMP";
const HUMIDITY_PREFIX: &str = "HUMIDITY";
const LOCATION_PREFIX: &str = "LOCATION";

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = get_input_for_day(5);

    // Graph construction

    let splits = inputs.split("\n\n").collect::<Vec<_>>();
    let seed_input = splits[0];
    let seed_to_soil_input = splits[1];
    let soil_to_fertilizer_input = splits[2];
    let fertilizer_to_water_input = splits[3];
    let water_to_light_input = splits[4];
    let light_to_temp_input = splits[5];
    let temp_to_humidity_input = splits[6];
    let humidity_to_location_input = splits[7];

    let mut adj_list: std::collections::BTreeMap<String, (u64, String)> =
        std::collections::BTreeMap::new();

    seed_to_soil_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let soil_seed_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let soil_start = soil_seed_range[0].parse::<u64>().unwrap();
            let seed_start = soil_seed_range[1].parse::<u64>().unwrap();
            let range = soil_seed_range[2].parse::<u64>().unwrap();
            let seed = format!("{}|{:0>10}", SEED_PREFIX, seed_start);
            let soil = format!("{}|{:0>10}", SOIL_PREFIX, soil_start);
            adj_list.insert(seed, (range, soil));
        });

    soil_to_fertilizer_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let soil_fertilizer_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let fertilizer_start = soil_fertilizer_range[0].parse::<u64>().unwrap();
            let soil_start = soil_fertilizer_range[1].parse::<u64>().unwrap();
            let range = soil_fertilizer_range[2].parse::<u64>().unwrap();
            let soil = format!("{}|{:0>10}", SOIL_PREFIX, soil_start);
            let fertilizer = format!("{}|{:0>10}", FERTILIZER_PREFIX, fertilizer_start);
            adj_list.insert(soil, (range, fertilizer));
        });

    fertilizer_to_water_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let fertilizer_water_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let water_start = fertilizer_water_range[0].parse::<u64>().unwrap();
            let fertilizer_start = fertilizer_water_range[1].parse::<u64>().unwrap();
            let range = fertilizer_water_range[2].parse::<u64>().unwrap();
            let fertilizer = format!("{}|{:0>10}", FERTILIZER_PREFIX, fertilizer_start);
            let water = format!("{}|{:0>10}", WATER_PREFIX, water_start);
            adj_list.insert(fertilizer, (range, water));
        });

    water_to_light_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let water_light_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let light_start = water_light_range[0].parse::<u64>().unwrap();
            let water_start = water_light_range[1].parse::<u64>().unwrap();
            let range = water_light_range[2].parse::<u64>().unwrap();
            let water = format!("{}|{:0>10}", WATER_PREFIX, water_start);
            let light = format!("{}|{:0>10}", LIGHT_PREFIX, light_start);
            adj_list.insert(water, (range, light));
        });

    light_to_temp_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let light_temp_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let temp_start = light_temp_range[0].parse::<u64>().unwrap();
            let light_start = light_temp_range[1].parse::<u64>().unwrap();
            let range = light_temp_range[2].parse::<u64>().unwrap();
            let light = format!("{}|{:0>10}", LIGHT_PREFIX, light_start);
            let temp = format!("{}|{:0>10}", TEMP_PREFIX, temp_start);
            adj_list.insert(light, (range, temp));
        });

    temp_to_humidity_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let temp_humidity_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let humidity_start = temp_humidity_range[0].parse::<u64>().unwrap();
            let temp_start = temp_humidity_range[1].parse::<u64>().unwrap();
            let range = temp_humidity_range[2].parse::<u64>().unwrap();
            let temp = format!("{}|{:0>10}", TEMP_PREFIX, temp_start);
            let humidity = format!("{}|{:0>10}", HUMIDITY_PREFIX, humidity_start);
            adj_list.insert(temp, (range, humidity));
        });

    humidity_to_location_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let humidity_location_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let location_start = humidity_location_range[0].parse::<u64>().unwrap();
            let humidity_start = humidity_location_range[1].parse::<u64>().unwrap();
            let range = humidity_location_range[2].parse::<u64>().unwrap();
            let humidity = format!("{}|{:0>10}", HUMIDITY_PREFIX, humidity_start);
            let location = format!("{}|{:0>10}", LOCATION_PREFIX, location_start);
            adj_list.insert(humidity, (range, location));
        });

    let initial_seeds = seed_input
        .split(": ") // Get right side of colon
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| format!("{}|{:0>10}", SEED_PREFIX, s))
        .collect::<Vec<_>>();

    // bfs

    let mut lowest_location = u64::MAX;
    let mut frontier = std::collections::VecDeque::new();

    for seed in initial_seeds {
        frontier.push_back(seed);
    }

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();

        if current.starts_with(LOCATION_PREFIX) {
            let location = current.split("|").nth(1).unwrap().parse::<u64>().unwrap();
            lowest_location = lowest_location.min(location);
        }

        let cursor = adj_list.upper_bound(std::ops::Bound::Included(&current));
        let Some((source, (range, dest))) = cursor.peek_prev() else {
            continue;
        };

        let source_name = source.split("|").next().unwrap();
        let source_distance = source.split("|").nth(1).unwrap().parse::<u64>().unwrap();
        let dest_name = dest.split("|").next().unwrap();
        let dest_distance = dest.split("|").nth(1).unwrap().parse::<u64>().unwrap();
        let current_name = current.split("|").next().unwrap();
        let current_distance = current.split("|").nth(1).unwrap().parse::<u64>().unwrap();

        if current_name != source_name {
            continue;
        }

        let delta = current_distance - source_distance;

        if delta < *range {
            frontier.push_back(format!("{}|{:0>10}", dest_name, dest_distance + delta));
        } else {
            frontier.push_back(format!("{}|{:0>10}", dest_name, current_distance));
        }
    }

    println!("lowest_location {}", lowest_location);

    Ok(())
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = get_input_for_day(5);

    // Graph construction

    let splits = inputs.split("\n\n").collect::<Vec<_>>();
    let seed_input = splits[0];
    let seed_to_soil_input = splits[1];
    let soil_to_fertilizer_input = splits[2];
    let fertilizer_to_water_input = splits[3];
    let water_to_light_input = splits[4];
    let light_to_temp_input = splits[5];
    let temp_to_humidity_input = splits[6];
    let humidity_to_location_input = splits[7];

    let mut adj_list: std::collections::BTreeMap<String, (u64, String)> =
        std::collections::BTreeMap::new();

    seed_to_soil_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let soil_seed_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let soil_start = soil_seed_range[0].parse::<u64>().unwrap();
            let seed_start = soil_seed_range[1].parse::<u64>().unwrap();
            let range = soil_seed_range[2].parse::<u64>().unwrap();
            let seed = format!("{}|{:0>10}", SEED_PREFIX, seed_start);
            let soil = format!("{}|{:0>10}", SOIL_PREFIX, soil_start);
            adj_list.insert(seed, (range, soil));
        });

    soil_to_fertilizer_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let soil_fertilizer_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let fertilizer_start = soil_fertilizer_range[0].parse::<u64>().unwrap();
            let soil_start = soil_fertilizer_range[1].parse::<u64>().unwrap();
            let range = soil_fertilizer_range[2].parse::<u64>().unwrap();
            let soil = format!("{}|{:0>10}", SOIL_PREFIX, soil_start);
            let fertilizer = format!("{}|{:0>10}", FERTILIZER_PREFIX, fertilizer_start);
            adj_list.insert(soil, (range, fertilizer));
        });

    fertilizer_to_water_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let fertilizer_water_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let water_start = fertilizer_water_range[0].parse::<u64>().unwrap();
            let fertilizer_start = fertilizer_water_range[1].parse::<u64>().unwrap();
            let range = fertilizer_water_range[2].parse::<u64>().unwrap();
            let fertilizer = format!("{}|{:0>10}", FERTILIZER_PREFIX, fertilizer_start);
            let water = format!("{}|{:0>10}", WATER_PREFIX, water_start);
            adj_list.insert(fertilizer, (range, water));
        });

    water_to_light_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let water_light_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let light_start = water_light_range[0].parse::<u64>().unwrap();
            let water_start = water_light_range[1].parse::<u64>().unwrap();
            let range = water_light_range[2].parse::<u64>().unwrap();
            let water = format!("{}|{:0>10}", WATER_PREFIX, water_start);
            let light = format!("{}|{:0>10}", LIGHT_PREFIX, light_start);
            adj_list.insert(water, (range, light));
        });

    light_to_temp_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let light_temp_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let temp_start = light_temp_range[0].parse::<u64>().unwrap();
            let light_start = light_temp_range[1].parse::<u64>().unwrap();
            let range = light_temp_range[2].parse::<u64>().unwrap();
            let light = format!("{}|{:0>10}", LIGHT_PREFIX, light_start);
            let temp = format!("{}|{:0>10}", TEMP_PREFIX, temp_start);
            adj_list.insert(light, (range, temp));
        });

    temp_to_humidity_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let temp_humidity_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let humidity_start = temp_humidity_range[0].parse::<u64>().unwrap();
            let temp_start = temp_humidity_range[1].parse::<u64>().unwrap();
            let range = temp_humidity_range[2].parse::<u64>().unwrap();
            let temp = format!("{}|{:0>10}", TEMP_PREFIX, temp_start);
            let humidity = format!("{}|{:0>10}", HUMIDITY_PREFIX, humidity_start);
            adj_list.insert(temp, (range, humidity));
        });

    humidity_to_location_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .for_each(|l| {
            let humidity_location_range = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let location_start = humidity_location_range[0].parse::<u64>().unwrap();
            let humidity_start = humidity_location_range[1].parse::<u64>().unwrap();
            let range = humidity_location_range[2].parse::<u64>().unwrap();
            let humidity = format!("{}|{:0>10}", HUMIDITY_PREFIX, humidity_start);
            let location = format!("{}|{:0>10}", LOCATION_PREFIX, location_start);
            adj_list.insert(humidity, (range, location));
        });

    let initial_seeds = seed_input
        .split(": ") // Get right side of colon
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let initial_seeds = initial_seeds
        .clone()
        .into_iter()
        .step_by(2)
        .map(|s| format!("{}|{:0>10}", SEED_PREFIX, s))
        .zip(initial_seeds.into_iter().skip(1).step_by(2))
        .collect::<Vec<_>>();

    // bfs

    let mut lowest_location = u64::MAX;
    let mut frontier = std::collections::VecDeque::new();

    for seed in initial_seeds {
        frontier.push_back(seed);
    }

    while !frontier.is_empty() {
        let (current, cur_range) = frontier.pop_front().unwrap();
        println!("current {} {}", current, cur_range);

        let current_name = current.split("|").next().unwrap();
        let current_start = current.split("|").nth(1).unwrap().parse::<u64>().unwrap();

        if current_name == LOCATION_PREFIX {
            lowest_location = lowest_location.min(current_start);
        }

        let cursor = adj_list.upper_bound(std::ops::Bound::Included(&format!(
            "{}|{:0>10}",
            current_name,
            current_start + cur_range
        )));
        let Some((source, (range, dest))) = cursor.peek_prev() else {
            continue;
        };

        let source_name = source.split("|").next().unwrap();
        let source_start = source.split("|").nth(1).unwrap().parse::<u64>().unwrap();
        let dest_name = dest.split("|").next().unwrap();
        let dest_start = dest.split("|").nth(1).unwrap().parse::<u64>().unwrap();

        if current_name != source_name {
            continue;
        }

        if source_start <= current_start && source_start + range >= current_start + cur_range {
            // Range fully covers current range
            // # # # # # # # # # # # # # # # # # # # # #
            //         ^CS                   ^CE
            //   ^SS                                 ^SE
            let delta = current_start - source_start;
            frontier.push_back((
                format!("{}|{:0>10}", dest_name, dest_start + delta),
                cur_range,
            ));
        } else {
            // Range starts after current range
            // # # # # # # # # # # # # # # # # # # # # #
            // ^CS                                 ^CE
            //         ^SS                     ^SE
            let start = max(current_start, source_start);
            let end = min(current_start + cur_range, source_start + range);
            if start < end {
                let _range = end - start;
                let delta = start - current_start;

                frontier.push_back((format!("{}|{:0>10}", dest_name, dest_start + delta), _range));
            }
        }
    }

    println!("lowest_location {}", lowest_location);

    Ok(())
}
