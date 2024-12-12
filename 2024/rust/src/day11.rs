use crate::{get_input_for_day, get_test_input};

pub fn part_one() {
    let mut nums = get_input_for_day(11)
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    fn apply_rules(nums: Vec<String>) -> Vec<String> {
        nums.into_iter()
            .flat_map(|s| {
                if s == "0" {
                    return vec!["1".to_owned()];
                }

                if s.len() % 2 == 0 {
                    let first_half = s.as_str()[0..s.len() / 2].parse::<u64>().unwrap();
                    let second_half = s.as_str()[s.len() / 2..].parse::<u64>().unwrap();
                    return vec![format!("{}", first_half), format!("{}", second_half)];
                }

                vec![format!("{}", s.parse::<u64>().unwrap() * 2024)]
            })
            .collect::<Vec<_>>()
    }

    for _ in 0..25 {
        nums = apply_rules(nums);
    }

    println!("{}", nums.len());
}

pub fn part_two() {}
