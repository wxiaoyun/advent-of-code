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
        .map(|(mut argmt, grp)| permutate(argmt, grp))
        .sum::<u64>();

    println!("Part One: {}", res);

    Ok(())
}

fn verify(arr: &[char], grouping: &[i64]) -> bool {
    let mut grouping = std::collections::VecDeque::from(Vec::from(grouping));

    let mut i = 0;
    while i < arr.len() && arr[i] == '.' {
        i += 1;
    }

    while i < arr.len() {
        match arr[i] {
            '.' => {
                let first = *grouping.front().unwrap_or(&0);
                if first != 0 {
                    return false;
                }
                grouping.pop_front();
                while i < arr.len() && arr[i] == '.' {
                    i += 1;
                }
            }
            '#' => {
                *match grouping.get_mut(0) {
                    Some(x) => x,
                    None => {
                        return false;
                    }
                } -= 1;
                i += 1;
            }
            _ => panic!("Input vec is not fully permutated: {:?}", arr),
        }
    }

    for g in grouping {
        if g != 0 {
            return false;
        }
    }

    true
}

pub fn permutate(mut arr: Vec<char>, grouping: Vec<i64>) -> u64 {
    fn helper(i: usize, arr: &mut Vec<char>, grouping: &Vec<i64>) -> u64 {
        if i >= arr.len() {
            return if verify(arr, grouping) { 1 } else { 0 };
        };

        let '?' = arr[i] else {
            return helper(i + 1, arr, grouping);
        };

        let mut res = 0;
        for c in ['.', '#'] {
            let original = arr[i];
            arr[i] = c;
            res += helper(i + 1, arr, grouping);
            arr[i] = original;
        }
        res
    };

    helper(0, &mut arr, &grouping)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        assert!(verify(&['#', '.', '.', '#'], &[1, 1]));
        assert!(verify(&['#', '#', '.', '#'], &[2, 1]));
        assert!(verify(&['#', '.', '.', '.'], &[1]));
        assert!(verify(&['.', '.', '.', '#'], &[1]));
    }
}
