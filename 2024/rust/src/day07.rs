use std::{ascii, collections::VecDeque};

use crate::{get_input_for_day, get_test_input};

pub fn part_one() {
    fn is_possible(target: i64, values: &mut Vec<i64>) -> bool {
        if values.is_empty() {
            return false;
        }

        if values.len() == 1 && values[0] == target {
            return true;
        }

        // values.len > 1
        let last = values.pop().unwrap();

        // check addition
        let mut possible = false;
        possible = is_possible(target - last, values);

        // check multiplication if divisible
        if !possible && target / last * last == target {
            possible = is_possible(target / last, values)
        }

        values.push(last);
        possible
    }

    let res = get_input_for_day(7)
        .lines()
        .map(|l| {
            let mut parts = l.split(":");
            let target = parts.next().unwrap().parse::<i64>().unwrap();
            let mut values = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if is_possible(target, &mut values) {
                target
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("{}", res);
}

pub fn part_two() {
    fn is_possible(target: i64, values: &mut Vec<i64>) -> bool {
        if values.is_empty() {
            return false;
        }

        if values.len() == 1 && values[0] == target {
            return true;
        }

        // values.len > 1
        let last = values.pop().unwrap();

        // check addition
        let mut possible = false;
        possible = is_possible(target - last, values);

        // check multiplication if divisible
        if !possible && target / last * last == target {
            possible = is_possible(target / last, values)
        }

        if !possible {
            let last_str = format!("{}", last);
            let target_str = format!("{}", target);
            if target_str.ends_with(&last_str) {
                let pow = 10_i64.pow(last_str.len() as u32);
                possible = is_possible(target / pow, values);
            }
        }

        values.push(last);
        possible
    }

    let res = get_input_for_day(7)
        .lines()
        .map(|l| {
            let mut parts = l.split(":");
            let target = parts.next().unwrap().parse::<i64>().unwrap();
            let mut values = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if is_possible(target, &mut values) {
                target
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("{}", res);
}
