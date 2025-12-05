fn parse_input(input: impl AsRef<str>) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut parts = input.as_ref().split("\n\n");

    let ranges: Vec<_> = parts
        .next()
        .and_then(|ranges| {
            ranges
                .lines()
                .map(|l| {
                    let mut range = l.split('-');
                    let mut get_num = || range.next().and_then(|s| s.parse::<i64>().ok());
                    get_num().and_then(|l| get_num().map(|r| (l, r)))
                })
                .collect::<Option<_>>()
        })
        .unwrap();

    let queries: Vec<_> = parts
        .next()
        .and_then(|s| {
            s.lines()
                .map(|l| l.parse::<i64>().ok())
                .collect::<Option<_>>()
        })
        .unwrap();

    (ranges, queries)
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (mut ranges, mut queries) = parse_input(input);
    ranges.sort();

    let mut merged_ranges = Vec::with_capacity(ranges.len());
    ranges.push((i64::MAX, i64::MAX));
    let (mut prev_s, mut prev_e) = ranges[0];
    for (s, e) in ranges.into_iter().skip(1) {
        if prev_e < s {
            merged_ranges.push((prev_s, prev_e));
            (prev_s, prev_e) = (s, e);
            continue;
        }

        prev_e = prev_e.max(e);
    }

    queries.sort();
    let mut i = 0;
    let mut fresh_cnt = 0;
    'outer: for n in queries {
        while i < merged_ranges.len() {
            let (s, e) = merged_ranges[i];
            // n < s <= e
            if n < s {
                continue 'outer;
            }
            // s <= e < n
            if e < n {
                i += 1;
                continue;
            }

            // s <= n <= e
            fresh_cnt += 1;
            continue 'outer;
        }
    }

    fresh_cnt
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let (mut ranges, _) = parse_input(input);
    ranges.sort();

    let mut merged_ranges = Vec::with_capacity(ranges.len());
    ranges.push((i64::MAX, i64::MAX));
    let (mut prev_s, mut prev_e) = ranges[0];
    for (s, e) in ranges.into_iter().skip(1) {
        if prev_e < s {
            merged_ranges.push((prev_s, prev_e));
            (prev_s, prev_e) = (s, e);
            continue;
        }

        prev_e = prev_e.max(e);
    }

    merged_ranges.into_iter().map(|(l, r)| r - l + 1).sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn test_part1() {
        assert_eq!(3, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(14, super::part_two(TEST_INPUT));
    }
}
