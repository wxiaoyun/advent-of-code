use std::{collections::HashMap, vec};

use rayon::prelude::*;
use regex::Regex;

fn parse_input(input: impl AsRef<str>) -> (Vec<bool>, Vec<Vec<usize>>, Vec<usize>) {
    let indicator_reg = Regex::new(r"\[([#\.]+)\]").unwrap();
    let toggle_reg = Regex::new(r"\(([,0-9]+)\)").unwrap();
    let jolt_reg = Regex::new(r"\{([,0-9]+)\}").unwrap();

    let indicator: Vec<_> = indicator_reg
        .captures(input.as_ref())
        .and_then(|c| c.get(1))
        .unwrap()
        .as_str()
        .chars()
        .map(|c| c == '#')
        .collect();
    let toggle_groups: Vec<Vec<_>> = toggle_reg
        .captures_iter(input.as_ref())
        .map(|cap| {
            cap.get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .unwrap();
    let jolt: Vec<_> = jolt_reg
        .captures(input.as_ref())
        .and_then(|c| c.get(1))
        .unwrap()
        .as_str()
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .unwrap();

    (indicator, toggle_groups, jolt)
}

fn solve_one(
    indicator: &mut Vec<bool>,
    target: &Vec<bool>,
    toggle_groups: &Vec<Vec<usize>>,
    i: usize,
    toggles: i64,
) -> i64 {
    if i >= toggle_groups.len() {
        let is_same = indicator
            .iter()
            .copied()
            .zip(target.iter().copied())
            .fold(true, |prev, (cur, target)| prev && (cur == target));

        return if is_same { toggles } else { i64::MAX };
    }

    let mut best = i64::MAX;
    let toggle_group = &toggle_groups[i];

    // Skip
    best = best.min(solve_one(indicator, target, toggle_groups, i + 1, toggles));

    // Toggle
    for j in toggle_group.iter().copied() {
        indicator[j] = !indicator[j]
    }
    best = best.min(solve_one(
        indicator,
        target,
        toggle_groups,
        i + 1,
        toggles + 1,
    ));
    for j in toggle_group.iter().copied() {
        indicator[j] = !indicator[j]
    }

    best
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    input
        .as_ref()
        .lines()
        .map(|input| {
            let (target, toggle_groups, _) = parse_input(input);
            let mut tmp = vec![false; target.len()];
            solve_one(&mut tmp, &target, &toggle_groups, 0, 0)
        })
        .sum()
}

fn solve_two(
    dp: &mut HashMap<Vec<usize>, i64>,
    toggle_groups: &Vec<Vec<usize>>,
    target: &Vec<usize>,
    cur: Vec<usize>,
) -> i64 {
    if let Some(result) = dp.get(&cur).copied() {
        return result;
    }

    let is_same = target
        .iter()
        .copied()
        .zip(cur.iter().copied())
        .all(|(a, b)| a == b);
    if is_same {
        return 0;
    }

    let mut best = i64::MAX >> 1;

    'outer: for tg in toggle_groups.iter() {
        let mut new_state = cur.clone();
        for j in tg.iter().copied() {
            new_state[j] += 1;
            if new_state[j] > target[j] {
                continue 'outer;
            }
        }

        best = best.min(1 + solve_two(dp, toggle_groups, target, new_state));
    }

    dp.insert(cur, best);
    best
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    input
        .as_ref()
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|input| {
            let (_, toggle_groups, target_jolt) = parse_input(input);
            let mut dp = HashMap::new();
            let res = solve_two(
                &mut dp,
                &toggle_groups,
                &target_jolt,
                vec![0; target_jolt.len()],
            );
            println!("{:?}: {}", target_jolt, res);
            res
        })
        .sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn test_part1() {
        assert_eq!(7, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(33, super::part_two(TEST_INPUT));
    }
}
