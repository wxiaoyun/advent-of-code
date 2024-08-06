use std::cmp::Ordering::{self, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string("../input.txt")?;

    let hands_and_bid = inputs
        .lines()
        .map(|l| {
            let mut strs = l
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());
            let hand = strs.next().unwrap().chars().collect::<Vec<_>>();
            let bid = strs.next().unwrap().parse::<u32>().unwrap();
            (Hand::try_from(hand).unwrap(), bid)
        })
        .collect::<Vec<_>>();

    let lookup = hands_and_bid
        .clone()
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

    let mut hands = hands_and_bid
        .into_iter()
        .map(|(hand, _)| hand)
        .collect::<Vec<_>>();
    hands.sort_by(cmp_hand);

    let mut res = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        let bid = lookup.get(&hand).unwrap();
        let rank = i + 1;
        res += bid * rank as u32;
    }

    println!("Total: {}", res);

    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Hand {
    FiveOfAKind(Vec<char>),
    FourOfAKind(Vec<char>),
    FullHouse(Vec<char>),
    ThreeOfAKind(Vec<char>),
    TwoPair(Vec<char>),
    OnePair(Vec<char>),
    HighCard(Vec<char>),
}

impl TryFrom<Vec<char>> for Hand {
    type Error = &'static str;

    fn try_from(value: Vec<char>) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err("Hand must have 5 cards");
        }

        let mut counts = std::collections::HashMap::new();
        for c in value.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(match counts.as_slice() {
            [(_, 5)] => Hand::FiveOfAKind(value),
            [(_, 4), (_, 1)] => Hand::FourOfAKind(value),
            [(_, 3), (_, 2)] => Hand::FullHouse(value),
            [(_, 3), (_, 1), (_, 1)] => Hand::ThreeOfAKind(value),
            [(_, 2), (_, 2), (_, 1)] => Hand::TwoPair(value),
            [(_, 2), (_, 1), (_, 1), (_, 1)] => Hand::OnePair(value),
            _ => Hand::HighCard(value),
        })
    }
}

fn char_val(c: &char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn same_variant_cmp(a: &[char], b: &[char]) -> Ordering {
    a.iter()
        .zip(b.iter())
        .find_map(|(a, b)| match char_val(a).cmp(&char_val(b)) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => None,
        })
        .unwrap_or(Equal)
}

fn cmp_hand(a: &Hand, b: &Hand) -> Ordering {
    match (a, b) {
        (Hand::FiveOfAKind(a), Hand::FiveOfAKind(b)) => same_variant_cmp(a, b),
        (Hand::FourOfAKind(a), Hand::FourOfAKind(b)) => same_variant_cmp(a, b),
        (Hand::FullHouse(a), Hand::FullHouse(b)) => same_variant_cmp(a, b),
        (Hand::ThreeOfAKind(a), Hand::ThreeOfAKind(b)) => same_variant_cmp(a, b),
        (Hand::TwoPair(a), Hand::TwoPair(b)) => same_variant_cmp(a, b),
        (Hand::OnePair(a), Hand::OnePair(b)) => same_variant_cmp(a, b),
        (Hand::HighCard(a), Hand::HighCard(b)) => same_variant_cmp(a, b),

        (Hand::FiveOfAKind(_), _) => Greater, // The other side cannot be a five of a kind or better
        (_, Hand::FiveOfAKind(_)) => Less,

        (Hand::FourOfAKind(_), _) => Greater, // The other side cannot be a four of a kind or better
        (_, Hand::FourOfAKind(_)) => Less,

        (Hand::FullHouse(_), _) => Greater, // The other side cannot be a full house or better
        (_, Hand::FullHouse(_)) => Less,

        (Hand::ThreeOfAKind(_), _) => Greater, // The other side cannot be a three of a kind or better
        (_, Hand::ThreeOfAKind(_)) => Less,

        (Hand::TwoPair(_), _) => Greater, // The other side cannot be a two pair or better
        (_, Hand::TwoPair(_)) => Less,

        (Hand::OnePair(_), _) => Greater, // The other side cannot be a one pair or better
        (_, Hand::OnePair(_)) => Less,
    }
}

fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string("../input.txt")?;

    let hands_and_bid = inputs
        .lines()
        .map(|l| {
            let mut strs = l
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());
            let hand = strs.next().unwrap().chars().collect::<Vec<_>>();
            let bid = strs.next().unwrap().parse::<u32>().unwrap();
            (Hand2::try_from(hand).unwrap(), bid)
        })
        .collect::<Vec<_>>();

    let lookup = hands_and_bid
        .clone()
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

    let mut hands = hands_and_bid
        .into_iter()
        .map(|(hand, _)| hand)
        .collect::<Vec<_>>();
    hands.sort_by(cmp_hand2);

    let mut res = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        let bid = lookup.get(&hand).unwrap();
        let rank = i + 1;
        res += bid * rank as u32;
    }

    println!("Total: {}", res);

    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Hand2 {
    FiveOfAKind(Vec<char>),
    FourOfAKind(Vec<char>),
    FullHouse(Vec<char>),
    ThreeOfAKind(Vec<char>),
    TwoPair(Vec<char>),
    OnePair(Vec<char>),
    HighCard(Vec<char>),
}

impl TryFrom<Vec<char>> for Hand2 {
    type Error = &'static str;

    fn try_from(value: Vec<char>) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err("Hand must have 5 cards");
        }

        let mut counts = std::collections::HashMap::new();
        for c in value.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let j_count = counts.get(&'J').copied().unwrap_or(0);
        counts.entry(&'J').and_modify(|c| *c -= j_count); // Remove the J's from the counts

        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts[0].1 += j_count; // Add the J's back in as wildcards

        Ok(match counts.as_slice() {
            [(_, 5)] | [(_, 5), _] => Hand2::FiveOfAKind(value),
            [(_, 4), (_, 1)] | [(_, 4), (_, 1), _] => Hand2::FourOfAKind(value),
            [(_, 3), (_, 2)] | [(_, 3), (_, 2), _] => Hand2::FullHouse(value),
            [(_, 3), (_, 1), (_, 1)] | [(_, 3), (_, 1), (_, 1), _] => Hand2::ThreeOfAKind(value),
            [(_, 2), (_, 2), (_, 1)] | [(_, 2), (_, 2), (_, 1), _] => Hand2::TwoPair(value),
            [(_, 2), (_, 1), (_, 1), (_, 1)] | [(_, 2), (_, 1), (_, 1), (_, 1), _] => {
                Hand2::OnePair(value)
            }
            _ => Hand2::HighCard(value),
        })
    }
}

fn char_val2(c: &char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => -1,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn same_variant_cmp2(a: &[char], b: &[char]) -> Ordering {
    a.iter()
        .zip(b.iter())
        .find_map(|(a, b)| match char_val2(a).cmp(&char_val2(b)) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => None,
        })
        .unwrap_or(Equal)
}

fn cmp_hand2(a: &Hand2, b: &Hand2) -> Ordering {
    match (a, b) {
        (Hand2::FiveOfAKind(a), Hand2::FiveOfAKind(b)) => same_variant_cmp2(a, b),
        (Hand2::FourOfAKind(a), Hand2::FourOfAKind(b)) => same_variant_cmp2(a, b),
        (Hand2::FullHouse(a), Hand2::FullHouse(b)) => same_variant_cmp2(a, b),
        (Hand2::ThreeOfAKind(a), Hand2::ThreeOfAKind(b)) => same_variant_cmp2(a, b),
        (Hand2::TwoPair(a), Hand2::TwoPair(b)) => same_variant_cmp2(a, b),
        (Hand2::OnePair(a), Hand2::OnePair(b)) => same_variant_cmp2(a, b),
        (Hand2::HighCard(a), Hand2::HighCard(b)) => same_variant_cmp2(a, b),

        (Hand2::FiveOfAKind(_), _) => Greater, // The other side cannot be a five of a kind or better
        (_, Hand2::FiveOfAKind(_)) => Less,

        (Hand2::FourOfAKind(_), _) => Greater, // The other side cannot be a four of a kind or better
        (_, Hand2::FourOfAKind(_)) => Less,

        (Hand2::FullHouse(_), _) => Greater, // The other side cannot be a full house or better
        (_, Hand2::FullHouse(_)) => Less,

        (Hand2::ThreeOfAKind(_), _) => Greater, // The other side cannot be a three of a kind or better
        (_, Hand2::ThreeOfAKind(_)) => Less,

        (Hand2::TwoPair(_), _) => Greater, // The other side cannot be a two pair or better
        (_, Hand2::TwoPair(_)) => Less,

        (Hand2::OnePair(_), _) => Greater, // The other side cannot be a one pair or better
        (_, Hand2::OnePair(_)) => Less,
    }
}
