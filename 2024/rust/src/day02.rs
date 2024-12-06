use std::collections::{self, hash_map::Entry, LinkedList, VecDeque};

use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let res = get_input_for_day(2)
        .split("\n")
        .map(|l| {
            let mut report = l
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if report.len() < 2 {
                return 1;
            }

            let delta = report[1] - report[0];
            if delta == 0 || !(-3..=3).contains(&delta) {
                return 0;
            }
            let is_inc = delta > 0;

            for i in 1..report.len() {
                let delta = report[i] - report[i - 1];
                if delta == 0 || !(-3..=3).contains(&delta) {
                    return 0;
                }

                let check_inc = delta > 0;

                if is_inc != check_inc {
                    return 0;
                }
            }

            1
        })
        .sum::<u32>();

    println!("{}", res);

    Ok(())
}

pub fn part_two() -> Result {
    let res = get_input_for_day(2)
        .split("\n")
        .map(|l| {
            let mut report = l
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<VecDeque<_>>();

            let mut inc_ok = true; // check inc
            let mut prev = report[0];
            let mut errors = 0;
            for &cur in report.iter().skip(1) {
                let delta = cur - prev;
                if (1..=3).contains(&delta) {
                    prev = cur;
                } else {
                    errors += 1;
                    if errors > 1 {
                        inc_ok = false;
                        break;
                    }
                }
            }

            let mut dec_ok = true; // check dec
            prev = report[0];
            errors = 0;
            for &cur in report.iter().skip(1) {
                let delta = cur - prev;
                if (-3..0).contains(&delta) {
                    prev = cur;
                } else {
                    // Ignore cur
                    errors += 1;
                    if errors > 1 {
                        dec_ok = false;
                        break;
                    }
                }
            }

            if inc_ok || dec_ok {
                println!("OK {:?}", report);
                1
            } else {
                println!("BAD {:?}", report);
                0
            }
        })
        .sum::<u32>();

    println!("{}", res);

    Ok(())
}
