use std::collections::{BinaryHeap, HashSet};

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mat = parse_input(input);
    let (n, m) = (mat.len(), mat[0].len());
    let dst = (n - 1, m - 1);

    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.extend([RIGHT, DOWN].iter().map(|&dir| (0, (0, 0, 0u8, dir))));
    while let Some((dist, node)) = pq.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());

        let (r, c, step, dir) = node;
        if (r, c) == dst {
            return -dist;
        }

        for new_dir in [UP, RIGHT, DOWN, LEFT] {
            if (new_dir as i8 - dir as i8)
                .abs()
                .min(new_dir as i8 + 4 - dir as i8)
                > 1
            {
                continue;
            }

            let mut step = step;
            if new_dir == dir {
                step += 1;
            } else {
                step = 0;
            }
            if step >= 3 {
                continue;
            }

            let (r, c) = (r as i64, c as i64);
            let (rr, cc) = match new_dir {
                UP => (r - 1, c),
                RIGHT => (r, c + 1),
                DOWN => (r + 1, c),
                LEFT => (r, c - 1),
                _ => unreachable!(),
            };

            if rr.min(cc) >= 0 && rr < n as i64 && cc < m as i64 {
                let (rr, cc) = (rr as usize, cc as usize);
                pq.push((dist - mat[rr][cc], (rr, cc, step, new_dir)));
            }
        }
    }

    -1
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mat = parse_input(input);
    let (n, m) = (mat.len(), mat[0].len());
    let dst = (n - 1, m - 1);

    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.extend([RIGHT, DOWN].iter().map(|&dir| (0, (0, 0, 0u8, dir))));
    while let Some((dist, node)) = pq.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());

        let (r, c, step, dir) = node;
        if (r, c) == dst {
            return -dist;
        }

        for new_dir in [UP, RIGHT, DOWN, LEFT] {
            if (new_dir as i8 - dir as i8)
                .abs()
                .min(new_dir as i8 + 4 - dir as i8)
                > 1
            {
                continue;
            }

            let mut step = step;
            if new_dir == dir {
                step += 1;
            } else {
                if step >= 3 {
                    step = 0;
                } else {
                    continue;
                }
            }

            if step >= 10 {
                continue;
            }

            let (r, c) = (r as i64, c as i64);
            let (rr, cc) = match new_dir {
                UP => (r - 1, c),
                RIGHT => (r, c + 1),
                DOWN => (r + 1, c),
                LEFT => (r, c - 1),
                _ => unreachable!(),
            };

            if rr.min(cc) >= 0 && rr < n as i64 && cc < m as i64 {
                let (rr, cc) = (rr as usize, cc as usize);
                pq.push((dist - mat[rr][cc], (rr, cc, step, new_dir)));
            }
        }
    }

    -1
}

#[inline]
fn parse_input(input: impl AsRef<str>) -> Vec<Vec<i64>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect()
}
