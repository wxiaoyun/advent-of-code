use std::collections::HashMap;

fn parse_input(input: impl AsRef<str>) -> HashMap<String, Vec<String>> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            let mut parts = l.split(": ");
            let src = parts.next().map(|s| s.to_owned()).unwrap();
            let dest: Vec<String> = parts
                .next()
                .map(|s| s.split_ascii_whitespace().map(|s| s.to_owned()))
                .map(|it| it.collect())
                .unwrap();
            (src, dest)
        })
        .collect()
}

fn compute(
    dp: &mut HashMap<String, i64>,
    adj_list: &HashMap<String, Vec<String>>,
    target: &str,
    cur: &str,
) -> i64 {
    let cur_owned = cur.to_owned();
    if let Some(ways) = dp.get(&cur_owned).copied() {
        return ways;
    }

    if cur == target {
        return 1;
    }

    let mut ways = 0;
    for nb in adj_list.get(&cur_owned).unwrap_or(&vec![]) {
        ways += compute(dp, adj_list, target, nb);
    }

    dp.insert(cur.to_string(), ways);
    ways
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let adj_list = parse_input(input);
    let mut dp = HashMap::new();
    compute(&mut dp, &adj_list, "out", "you")
}

fn compute2(
    dp: &mut HashMap<(String, bool, bool), i64>,
    adj_list: &HashMap<String, Vec<String>>,
    target: &str,
    cur: &str,
    visited_dac: bool,
    visited_fft: bool,
) -> i64 {
    let cur_owned = cur.to_owned();
    let key = (cur_owned.clone(), visited_dac, visited_fft);
    if let Some(ways) = dp.get(&key).copied() {
        return ways;
    }

    if cur == target {
        if visited_dac && visited_fft {
            return 1;
        }
        return 0;
    }

    let mut visited_dac = visited_dac;
    if !visited_dac && cur == "dac" {
        visited_dac = true;
    }

    let mut visited_fft = visited_fft;
    if !visited_fft && cur == "fft" {
        visited_fft = true;
    }

    let mut ways = 0;
    for nb in adj_list.get(&cur_owned).unwrap_or(&vec![]) {
        ways += compute2(dp, adj_list, target, nb, visited_dac, visited_fft);
    }

    dp.insert(key, ways);
    ways
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let adj_list = parse_input(input);
    let mut dp = HashMap::new();
    compute2(&mut dp, &adj_list, "out", "svr", false, false)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("../../../input/2025_11_1.test.txt");


    #[test]
    fn test_part1() {
        assert_eq!(5, super::part_one(TEST_INPUT));
    }

    const TEST_INPUT2: &str = include_str!("../../../input/2025_11_2.test.txt");

    #[test]
    fn test_part2() {
        assert_eq!(2, super::part_two(TEST_INPUT2));
    }
}
