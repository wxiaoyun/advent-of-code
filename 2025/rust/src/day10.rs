use std::collections::HashMap;

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
            solve_one(
                &mut vec![false; target.len()],
                &target,
                &toggle_groups,
                0,
                0,
            )
        })
        .sum()
}

// Generate all possible patterns (combinations of toggle groups)
fn generate_patterns(
    toggle_groups: &[Vec<usize>],
    num_indicators: usize,
) -> HashMap<Vec<usize>, i64> {
    let mut patterns = HashMap::new();
    let num_buttons = toggle_groups.len();

    // Try all combinations of buttons, ordered by number of buttons used
    // This ensures we find the minimum cost for each pattern
    for pattern_len in 0..=num_buttons {
        // Generate all combinations of buttons of this length
        generate_combinations(num_buttons, pattern_len, &mut |buttons| {
            let mut pattern = vec![0; num_indicators];

            // Apply each button in this combination
            for &button_idx in buttons {
                for &idx in &toggle_groups[button_idx] {
                    pattern[idx] += 1;
                }
            }

            // Only keep the first occurrence (minimum cost due to ordering)
            patterns.entry(pattern).or_insert(pattern_len as i64);
        });
    }

    patterns
}

// Helper function to generate all combinations of k elements from 0..n
fn generate_combinations<F>(n: usize, k: usize, callback: &mut F)
where
    F: FnMut(&[usize]),
{
    let mut combo = vec![0; k];
    if k == 0 {
        callback(&[]);
        return;
    }

    fn generate_recursive<F>(
        n: usize,
        k: usize,
        start: usize,
        combo: &mut Vec<usize>,
        pos: usize,
        callback: &mut F,
    ) where
        F: FnMut(&[usize]),
    {
        if pos == k {
            callback(combo);
            return;
        }

        for i in start..=n - (k - pos) {
            combo[pos] = i;
            generate_recursive(n, k, i + 1, combo, pos + 1, callback);
        }
    }

    generate_recursive(n, k, 0, &mut combo, 0, callback);
}

// Solution from reddit post:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
fn solve_two(
    dp: &mut HashMap<Vec<usize>, i64>,
    patterns: &HashMap<Vec<usize>, i64>,
    target: &[usize],
) -> i64 {
    if target.iter().copied().sum::<usize>() == 0 {
        return 0;
    }

    if let Some(&cached) = dp.get(target) {
        return cached;
    }

    let mut answer = i64::MAX;

    // Try all patterns that match parity and are <= target
    for (pattern, &pattern_cost) in patterns.iter() {
        // Check if pattern is valid for this target
        let valid = pattern.iter().zip(target.iter()).all(|(p, t)| {
            p <= t && (p % 2 == t % 2) // Same parity and pattern <= target
        });

        if !valid {
            continue;
        }

        // Compute new goal: (target - pattern) / 2
        let new_goal: Vec<usize> = pattern
            .iter()
            .zip(target.iter())
            .map(|(p, t)| (t - p) / 2)
            .collect();

        let recursive_cost = solve_two(dp, patterns, &new_goal);

        if recursive_cost == i64::MAX {
            continue;
        }

        answer = answer.min(recursive_cost * 2 + pattern_cost)
    }

    dp.insert(target.to_vec(), answer);
    answer
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    input
        .as_ref()
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|input| {
            let (indicator, toggle_groups, target_jolt) = parse_input(input);
            let patterns = generate_patterns(&toggle_groups, indicator.len());
            solve_two(&mut HashMap::new(), &patterns, &target_jolt)
        })
        .sum()
}
