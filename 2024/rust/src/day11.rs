use std::collections;

use crate::{get_input_for_day, get_test_input};

#[derive(Clone, Debug)]
struct Node {
    val: String,
    prev: Option<usize>,
    next: Option<usize>,
}

pub fn part_one() {
    let nums = get_input_for_day(11)
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let n = nums.len();

    let mut heap = nums
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let prev = if i > 0 { Some(i - 1) } else { None };
            let next = if i < n - 1 { Some(i + 1) } else { None };
            Node { val: s, prev, next }
        })
        .collect::<Vec<_>>();

    fn apply_rules(heap: &mut Vec<Node>) {
        let mut current = 0;

        loop {
            let next = heap[current].next;

            if heap[current].val == "0" {
                "1".clone_into(&mut heap[current].val);
            } else if heap[current].val.len() % 2 == 0 {
                let s = heap[current].val.clone();
                let first_half = s.as_str()[0..s.len() / 2].parse::<u64>().unwrap();
                let second_half = s.as_str()[s.len() / 2..].parse::<u64>().unwrap();

                let a = Node {
                    val: format!("{}", first_half),
                    prev: heap[current].prev,
                    next: Some(heap.len()),
                };

                let b = Node {
                    val: format!("{}", second_half),
                    prev: Some(current),
                    next: heap[current].next,
                };

                heap[current] = a;
                heap.push(b);
            } else {
                let val = heap[current].val.clone().parse::<u64>().unwrap();
                heap[current].val = format!("{}", val * 2024);
            }

            let Some(next_idx) = next else {
                break;
            };
            current = next_idx;
        }
    }

    for i in 0..25 {
        apply_rules(&mut heap);
    }

    println!("{}", heap.len());
}

pub fn part_two() {
    let nums = get_input_for_day(11)
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    // What is the number of strings that A produces after blinking k times?
    fn blink(dp: &mut collections::HashMap<(String, u64), u64>, a: String, k: u64) -> u64 {
        if k == 0 {
            return 1;
        }

        let key = (a.clone(), k);

        if dp.contains_key(&key) {
            return *dp.get(&key).unwrap();
        }

        let mut stones = 0;
        if a == "0" {
            stones += blink(dp, "1".to_owned(), k - 1);
        } else if a.len() % 2 == 0 {
            let first_half = a.as_str()[0..a.len() / 2].parse::<u64>().unwrap();
            let second_half = a.as_str()[a.len() / 2..].parse::<u64>().unwrap();

            stones += blink(dp, format!("{}", first_half), k - 1);
            stones += blink(dp, format!("{}", second_half), k - 1);
        } else {
            let num = a.parse::<u64>().unwrap();
            stones += blink(dp, format!("{}", num * 2024), k - 1);
        }

        dp.insert(key, stones);
        stones
    }

    let mut stones = 0;
    let mut dp: collections::HashMap<(String, u64), u64> = collections::HashMap::new();
    nums.into_iter().for_each(|n| {
        stones += blink(&mut dp, n, 75);
    });

    println!("{}", stones);
}
