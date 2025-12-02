use std::collections::{self, hash_map::Entry};

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (mut list_a, mut list_b) = input
        .as_ref()
        .lines()
        .map(|l| {
            let mut nums = l.split_ascii_whitespace();
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    list_a.sort();
    list_b.sort();

    let res = list_a
        .into_iter()
        .zip(list_b)
        .fold(0, |prev, (a, b)| prev + (if a > b { a - b } else { b - a }));
    res as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let (mut list_a, mut list_b) = input
        .as_ref()
        .lines()
        .map(|l| {
            let mut nums = l.split_ascii_whitespace();
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    let mut dict_b = list_b
        .into_iter()
        .fold(collections::HashMap::new(), |mut map, n| {
            let e = map.entry(n).or_insert(0_u32);
            *e += 1;
            map
        });

    let res = list_a
        .into_iter()
        .fold(0_u32, |sum, n| sum + n * *dict_b.entry(n).or_default());
    res as i64
}
