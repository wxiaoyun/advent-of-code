use std::collections::{self, HashMap, HashSet};

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.split("\n\n");

    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut nums = l.split('|');
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .fold(
            collections::HashMap::new(),
            |mut prev: HashMap<u32, Vec<u32>>, (a, b)| {
                let mut e = prev.entry(a).or_default();
                e.push(b);
                prev
            },
        );

    let prints = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    'outer: for pts in prints {
        let mut seen: collections::HashSet<u32> = collections::HashSet::new();

        for n in &pts {
            let afters = match rules.get(n) {
                Some(v) => v.clone(),
                None => vec![],
            };

            for a in afters {
                if seen.contains(&a) {
                    continue 'outer;
                }
            }

            seen.insert(*n);
        }

        sum += pts[pts.len() / 2];
    }

    sum as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.split("\n\n");

    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut nums = l.split('|');
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .fold(
            collections::HashMap::new(),
            |mut prev: HashMap<u32, Vec<u32>>, (a, b)| {
                let mut e = prev.entry(a).or_default();
                e.push(b);
                prev
            },
        );

    let prints = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut incorrect: Vec<Vec<u32>> = vec![];
    'outer: for pts in prints {
        let mut seen: collections::HashSet<u32> = collections::HashSet::new();

        for n in &pts {
            let afters = match rules.get(n) {
                Some(v) => v.clone(),
                None => vec![],
            };

            for a in afters {
                if seen.contains(&a) {
                    incorrect.push(pts.clone());
                    continue 'outer;
                }
            }

            seen.insert(*n);
        }
    }

    let mut sum = 0;
    for mut pts in incorrect {
        'middle: loop {
            let mut seen: collections::HashMap<u32, usize> = collections::HashMap::new();

            for (i, &n) in pts.iter().enumerate() {
                let afters = match rules.get(&n) {
                    Some(v) => v.clone(),
                    None => vec![],
                };

                for a in afters {
                    if let Some(&j) = seen.get(&a) {
                        pts.swap(i, j);
                        continue 'middle;
                    }
                }

                seen.insert(n, i);
            }

            sum += pts[pts.len() / 2];
            break;
        }
    }

    sum as i64
}
