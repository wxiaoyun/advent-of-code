use std::collections::{self, LinkedList, VecDeque, hash_map::Entry};

fn is_safe_no_removal(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true; // Single-element or empty is trivially safe
    }

    let delta = report[1] - report[0];
    if delta == 0 || !(-3..=3).contains(&delta) {
        return false;
    }
    let is_increasing = delta > 0;

    for i in 2..report.len() {
        let d = report[i] - report[i - 1];
        if d == 0 || !(-3..=3).contains(&d) {
            return false;
        }
        // All differences must have the same sign pattern
        if (d > 0) != is_increasing {
            return false;
        }
    }

    true
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let res = input
        .as_ref()
        .lines()
        .map(|l| {
            let mut report = l
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if is_safe_no_removal(&report) { 1 } else { 0 }
        })
        .sum::<u32>();

    res as i64
}

fn is_safe_with_one_removal(report: &[i32]) -> bool {
    // If already safe, no need to remove anything
    if is_safe_no_removal(report) {
        return true;
    }

    // Try removing each element and check if the sequence becomes safe
    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        if is_safe_no_removal(&modified) {
            return true;
        }
    }

    false
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let res = input
        .as_ref()
        .lines()
        .map(|l| {
            let mut report = l
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if is_safe_with_one_removal(&report) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    res as i64
}
