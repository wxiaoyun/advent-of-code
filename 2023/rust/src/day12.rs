use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let res = get_input_for_day(12)
        .lines()
        .map(|l| {
            let mut split = l.split(" ");
            let arrangement = split.next().unwrap().chars().collect::<Vec<_>>();
            let grouping = split
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (arrangement, grouping)
        })
        .map(|(mut argmt, grp)| permutate(&argmt, &grp))
        .sum::<u64>();

    println!("Part One: {}", res);

    Ok(())
}

fn permutate(cfg: &[char], grp: &[i64]) -> u64 {
    if cfg.is_empty() {
        return if grp.is_empty() { 1 } else { 0 };
    }

    if grp.is_empty() {
        return if cfg.contains(&'#') { 0 } else { 1 };
    }

    let first = cfg[0];
    let mut result = 0;

    if ['.', '?'].contains(&first) {
        result += permutate(&cfg[1..], grp);
    }

    if ['#', '?'].contains(&first) {
        let l = grp[0] as usize;
        if cfg.len() >= l
            && cfg[0..l].iter().all(|&c| c != '.')
            && (cfg.len() == l || cfg[l] != '#')
        {
            result += permutate(
                if cfg.len() > l + 1 {
                    &cfg[l + 1..]
                } else {
                    &[]
                },
                &grp[1..],
            )
        }
    }

    result
}

pub fn part_two() -> Result {
    let mut permutate = Permutate::new();
    let res = get_input_for_day(12)
        .lines()
        .map(|l| {
            let mut split = l.split(" ");
            let arrangement = split.next().unwrap().chars().collect::<String>();
            let arrangement = (0..5)
                .map(|_| arrangement.clone())
                .collect::<Vec<_>>()
                .join("?")
                .chars()
                .collect::<Vec<_>>();
            let grouping = split
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let grouping = (0..5).flat_map(|_| grouping.clone()).collect::<Vec<_>>();
            (arrangement, grouping)
        })
        .map(|(mut argmt, grp)| permutate.permutate(&argmt, &grp))
        .sum::<u64>();

    println!("Part Two: {}", res);

    Ok(())
}

struct Permutate {
    dp: std::collections::HashMap<(Vec<char>, Vec<i64>), u64>,
}

impl Permutate {
    pub fn new() -> Self {
        Self {
            dp: std::collections::HashMap::new(),
        }
    }

    pub fn permutate(&mut self, cfg: &[char], grp: &[i64]) -> u64 {
        if cfg.is_empty() {
            return if grp.is_empty() { 1 } else { 0 };
        }

        if grp.is_empty() {
            return if cfg.contains(&'#') { 0 } else { 1 };
        }

        if let Some(&res) = self.dp.get(&(cfg.to_vec(), grp.to_vec())) {
            return res;
        }

        let first = cfg[0];
        let mut result = 0;

        if ['.', '?'].contains(&first) {
            result += self.permutate(&cfg[1..], grp);
        }

        if ['#', '?'].contains(&first) {
            let l = grp[0] as usize;
            if cfg.len() >= l
                && cfg[0..l].iter().all(|&c| c != '.')
                && (cfg.len() == l || cfg[l] != '#')
            {
                result += self.permutate(
                    if cfg.len() > l + 1 {
                        &cfg[l + 1..]
                    } else {
                        &[]
                    },
                    &grp[1..],
                )
            }
        }

        self.dp.insert((cfg.to_vec(), grp.to_vec()), result);
        result
    }
}
