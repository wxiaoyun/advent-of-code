use std::collections::VecDeque;

const ROLL: char = '@';

fn is_accessible(mat: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if mat[r][c] != ROLL {
        return false;
    }

    let start_r = if r == 0 { r } else { r - 1 };
    let end_r = if r + 1 >= mat.len() { r } else { r + 1 };
    let start_c = if c == 0 { c } else { c - 1 };
    let end_c = if c + 1 >= mat.len() { c } else { c + 1 };

    let mut roll_cnt = 0;
    for rr in start_r..=end_r {
        for cc in start_c..=end_c {
            if (r, c) != (rr, cc) && mat[rr][cc] == ROLL {
                roll_cnt += 1;
            }
        }
    }

    roll_cnt < 4
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mat: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut accessible_cnt = 0;
    let n = mat.len();
    let m = mat[0].len();
    for i in 0..n {
        for j in 0..m {
            if is_accessible(&mat, i, j) {
                accessible_cnt += 1
            }
        }
    }

    accessible_cnt
}

fn remove_and_notify_neighbor(
    mat: &mut Vec<Vec<char>>,
    work_queue: &mut VecDeque<(usize, usize)>,
    r: usize,
    c: usize,
) {
    mat[r][c] = '.';

    let start_r = if r == 0 { r } else { r - 1 };
    let end_r = if r + 1 >= mat.len() { r } else { r + 1 };
    let start_c = if c == 0 { c } else { c - 1 };
    let end_c = if c + 1 >= mat.len() { c } else { c + 1 };

    for rr in start_r..=end_r {
        for cc in start_c..=end_c {
            if mat[rr][cc] == ROLL {
                work_queue.push_back((rr, cc));
            }
        }
    }
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mut mat: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut work_queue = VecDeque::new();
    let n = mat.len();
    let m = mat[0].len();
    for i in 0..n {
        for j in 0..m {
            if is_accessible(&mat, i, j) {
                work_queue.push_back((i, j));
            }
        }
    }

    let mut accessible_cnt = 0;
    while let Some((r, c)) = work_queue.pop_front() {
        if is_accessible(&mat, r, c) {
            accessible_cnt += 1;
            remove_and_notify_neighbor(&mut mat, &mut work_queue, r, c);
        }
    }

    accessible_cnt
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("../../../input/2025_04_1.test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(13, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(43, super::part_two(TEST_INPUT));
    }
}
