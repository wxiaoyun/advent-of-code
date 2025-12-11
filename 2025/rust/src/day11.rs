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

pub fn part_two(_: impl AsRef<str>) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    #[test]
    fn test_part1() {
        assert_eq!(5, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, super::part_two(TEST_INPUT));
    }
}
