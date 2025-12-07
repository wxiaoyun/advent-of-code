use std::collections::{HashMap, HashSet};

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mat: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let ncol = mat[0].len();
    let mut beams = HashSet::with_capacity(ncol);
    let mut tmp = HashSet::with_capacity(ncol);
    mat[0].iter().copied().enumerate().for_each(|(j, ch)| {
        if ch == 'S' {
            beams.insert(j);
        }
    });

    let mut splits = 0;
    let mut i = 0;
    while i < mat.len() {
        tmp.clear();
        for j in beams.iter().copied() {
            if mat[i][j] != '^' {
                tmp.insert(j);
                continue;
            }

            splits += 1;
            if j >= 1 {
                tmp.insert(j - 1);
            }
            if j + 1 < ncol {
                tmp.insert(j + 1);
            }
        }

        std::mem::swap(&mut beams, &mut tmp);
        i += 1;
    }

    splits
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mat: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let ncol = mat[0].len();
    let mut beams = HashMap::with_capacity(ncol);
    let mut tmp = HashMap::with_capacity(ncol);
    mat[0].iter().copied().enumerate().for_each(|(j, ch)| {
        if ch == 'S' {
            beams.insert(j, 1);
        }
    });

    let mut i = 0;
    while i < mat.len() {
        tmp.clear();
        for (&j, &cnt) in beams.iter() {
            if mat[i][j] != '^' {
                *tmp.entry(j).or_insert(0) += cnt;
                continue;
            }

            if j >= 1 {
                *tmp.entry(j - 1).or_insert(0) += cnt;
            }
            if j + 1 < ncol {
                *tmp.entry(j + 1).or_insert(0) += cnt;
            }
        }

        std::mem::swap(&mut beams, &mut tmp);
        i += 1;
    }

    beams.values().sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_part1() {
        assert_eq!(21, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(40, super::part_two(TEST_INPUT));
    }
}
