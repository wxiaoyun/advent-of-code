use std::collections::HashMap;

use crate::{get_input_for_day, Result};

pub fn part_one() -> Result {
    let inputs = get_input_for_day(1);

    let calibration_sum = inputs
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect::<Vec<Vec<char>>>()
        .into_iter()
        .map(|l| {
            let first_digit = l.first().and_then(|c| c.to_digit(10)).unwrap(); // Safe given input
            let last_digit = l.last().and_then(|c| c.to_digit(10)).unwrap(); // Safe given input
            first_digit * 10 + last_digit
        })
        .sum::<u32>();

    println!("Sum of first and last digits: {}", calibration_sum);
    Ok(())
}

pub fn part_two() -> Result {
    use trie_rs::*;

    let eng_to_num = std::collections::HashMap::from([
        ("one", 1_u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut builder = TrieBuilder::new();
    for &num in eng_to_num.keys() {
        builder.push(num)
    }
    let trie = builder.build();

    let mut builder = TrieBuilder::new();
    for num in eng_to_num
        .keys()
        .map(|s| s.chars().rev().collect::<String>())
    {
        builder.push(num)
    }
    let trie_rev = builder.build();

    let get_num = |i: usize, chars: &[char], trie: &Trie<u8>| -> Option<u32> {
        if chars[i].is_ascii_digit() {
            return Some(chars[i].to_digit(10).unwrap());
        }

        let mut result: Option<String> = trie
            .common_prefix_search(chars[i..].iter().collect::<String>())
            .last();

        match result {
            None => None,
            Some(best_match) => {
                if let Some(n) = eng_to_num.get(best_match.as_str()).copied() {
                    return Some(n);
                }

                if let Some(n) = eng_to_num
                    .get(best_match.chars().rev().collect::<String>().as_str())
                    .copied()
                {
                    return Some(n);
                }

                panic!("No number found");
            }
        }
    };

    let calibration_sum = get_input_for_day(1)
        .lines()
        .map(|l| {
            let chars = l.chars();
            let num1 =
                (0..l.len()).find_map(|i| get_num(i, &chars.clone().collect::<Vec<_>>(), &trie));
            let num2 = (0..l.len())
                .find_map(|i| get_num(i, &chars.clone().rev().collect::<Vec<_>>(), &trie_rev));
            num1.unwrap() * 10 + num2.unwrap()
        })
        .sum::<u32>();

    println!("Sum of first and last digits: {}", calibration_sum);

    Ok(())
}
