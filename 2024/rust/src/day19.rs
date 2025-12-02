use std::collections::{HashMap, HashSet};

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.split("\n\n");

    let patterns = input
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>();
    let targets = input
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let mut dp = HashMap::new();

    fn is_possible(
        target: String,
        patterns: &HashSet<String>,
        dp: &mut HashMap<String, bool>,
    ) -> bool {
        if dp.contains_key(&target) {
            return *dp.get(&target).unwrap();
        }

        if patterns.contains(&target) {
            dp.insert(target, true);
            return true;
        }

        for i in 1..target.len() {
            let left = &target[..i];
            let right = &target[i..];

            if is_possible(left.to_owned(), patterns, dp)
                && is_possible(right.to_owned(), patterns, dp)
            {
                dp.insert(target, true);
                return true;
            }
        }

        dp.insert(target, false);
        false
    }

    let result = targets
        .into_iter()
        .filter(|t| is_possible(t.clone(), &patterns, &mut dp))
        .count();

    result as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.split("\n\n");

    let patterns = input
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>();
    let targets = input
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let mut dp = HashMap::new();

    fn is_possible(
        target: String,
        patterns: &HashSet<String>,
        dp: &mut HashMap<String, u64>,
    ) -> u64 {
        if dp.contains_key(&target) {
            return *dp.get(&target).unwrap();
        }

        let mut total_permutations = 0;
        if patterns.contains(&target) {
            total_permutations += 1;
        }

        for i in 1..target.len() {
            let left = &target[..i];
            let right = &target[i..];

            if patterns.contains(left) {
                let right_permutations = is_possible(right.to_owned(), patterns, dp);
                total_permutations += right_permutations;
            }
        }

        dp.insert(target, total_permutations);
        total_permutations
    }

    let result = targets
        .into_iter()
        .map(|t| is_possible(t.clone(), &patterns, &mut dp))
        .sum::<u64>();

    result as i64
}
