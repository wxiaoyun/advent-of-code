use crate::{get_input_for_day, Result};

pub fn part_one() -> Result {
    fn predict_next(vals: Vec<i64>) -> i64 {
        let mut is_all_zeros = true;
        let mut delta = (0..(vals.len() - 1))
            .map(|i| {
                let diff = vals[i + 1] - vals[i];
                if diff != 0 {
                    is_all_zeros = false;
                }
                diff
            })
            .collect::<Vec<_>>();

        *vals.last().unwrap() + if is_all_zeros { 0 } else { predict_next(delta) }
    }

    let res = get_input_for_day(9)
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(predict_next)
        .sum::<i64>();

    println!("Sum of predictions: {}", res);

    Ok(())
}
