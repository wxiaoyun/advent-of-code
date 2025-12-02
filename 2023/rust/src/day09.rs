pub fn part_one(input: impl AsRef<str>) -> i64 {
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

        let last = *vals.last().unwrap();
        drop(vals);

        last + if is_all_zeros { 0 } else { predict_next(delta) }
    }

    let res = input
        .as_ref()
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

    res
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
  fn predict_prev(vals: Vec<i64>) -> i64 {
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

    let first = *vals.first().unwrap();
    drop(vals);

    first - if is_all_zeros { 0 } else { predict_prev(delta) }
}

let res = input
    .as_ref()
    .lines()
    .map(|l| {
        l.split_ascii_whitespace()
            .map(|s| s.trim())
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    })
    .map(predict_prev)
    .sum::<i64>();

res
}
