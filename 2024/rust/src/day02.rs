use std::collections::{self, hash_map::Entry, LinkedList};

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
    // let res = get_test_input(2)
    //     .split("\n")
    //     .map(|l| {
    //         let mut report = l
    //             .split_ascii_whitespace()
    //             .map(|n| n.parse::<i32>().unwrap())
    //             .collect::<LinkedList<_>>();

    //         if report.len() < 2 {
    //             return 1;
    //         }

    //         let mut reportc = report.clone();
    //         let is_inc = true;

    //         while report.tail.is_some() && report.tail.unwrap().tail



    //         1
    //     })
    //     .sum::<u32>();

    // println!("{}", res);

    Ok(())
}
